<script lang="ts">
  import { onMount } from 'svelte'
  import JSZip from 'jszip'

  let generatePdf: (json: string, template: string) => Uint8Array
  let generateTyp: (json: string, template: string) => string

  let jsonFile = $state<File | null>(null)
  let entries = $state<unknown[]>([])
  let templateFiles = $state<File[]>([])
  let templates = $state<Map<string, string>>(new Map())
  let processing = $state(false)
  let progress = $state('')
  let error = $state('')
  let jsonDragOver = $state(false)
  let templateDragOver = $state(false)

  onMount(async () => {
    const mod: any = await import('./wasm/tambo_wasm.js')
    generatePdf = mod.generate_pdf
    generateTyp = mod.generate_standalone_typ
    await mod.default()
  })

  function sanitize(str: string): string {
    return str.trim().toLowerCase().replace(/[\s_]/g, '-').replace(/[^a-z0-9-]/g, '')
  }

  async function loadJson(file: File) {
    jsonFile = file
    try {
      const text = await file.text()
      const data = JSON.parse(text)
      entries = Array.isArray(data) ? data : [data]
      error = ''
    } catch (e) {
      error = `Erreur de lecture JSON : ${e}`
      entries = []
    }
  }

  async function loadTemplate(file: File) {
    const name = sanitize(file.name.replace(/\.typ$/i, ''))
    const content = await file.text()
    templates.set(name, content)
    templateFiles = [...templateFiles, file]
  }

  function removeTemplate(file: File) {
    const name = sanitize(file.name.replace(/\.typ$/i, ''))
    templates.delete(name)
    templateFiles = templateFiles.filter((f) => f !== file)
  }

  function handleJsonDrop(e: DragEvent) {
    e.preventDefault()
    jsonDragOver = false
    const file = e.dataTransfer?.files[0]
    if (file?.name.endsWith('.json')) loadJson(file)
  }

  function handleTemplateDrop(e: DragEvent) {
    e.preventDefault()
    templateDragOver = false
    const files = Array.from(e.dataTransfer?.files ?? [])
    for (const f of files) {
      if (f.name.endsWith('.typ')) loadTemplate(f)
    }
  }

  function onJsonInput(e: Event) {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (file) loadJson(file)
  }

  function onTemplateInput(e: Event) {
    const files = Array.from((e.target as HTMLInputElement).files ?? [])
    for (const f of files) loadTemplate(f)
  }

  async function generate() {
    processing = true
    progress = ''
    error = ''

    const zip = new JSZip()
    let count = 0
    let skipped = 0

    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i] as Record<string, unknown>
      const groupe: unknown = entry.groupe
      if (!groupe) { skipped++; continue }

      const templateName = sanitize(String(groupe))
      const template = templates.get(templateName)
      if (!template) { skipped++; continue }

      progress = `(${i + 1}/${entries.length}) ${groupe}`
      const jsonStr = JSON.stringify(entry)
      const pdf = generatePdf(jsonStr, template)
      const typ = generateTyp(jsonStr, template)
      zip.file(`${templateName}.pdf`, pdf)
      zip.file(`${templateName}.typ`, typ)
      count++
    }

    if (count === 0) {
      error = 'Aucun fichier généré. Vérifiez que les templates correspondent aux entrées.'
      processing = false
      return
    }

    const blob = await zip.generateAsync({ type: 'blob' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'tambo-output.zip'
    a.click()
    URL.revokeObjectURL(url)

    progress = `${count} fichier(s) généré(s)${skipped > 0 ? ` (${skipped} ignoré(s))` : ''}`
    processing = false
  }
</script>

<div class="app">
  <h1>tambo</h1>

  <div
    class="dropzone"
    class:has={jsonFile !== null}
    class:dragover={jsonDragOver}
    role="button"
    tabindex="0"
    onclick={() => document.getElementById('json-input')?.click()}
    ondragover={(e) => { e.preventDefault(); jsonDragOver = true }}
    ondragleave={() => { jsonDragOver = false }}
    ondrop={handleJsonDrop}
    onkeydown={(e) => { if (e.key === 'Enter') document.getElementById('json-input')?.click() }}
  >
    {#if jsonFile}
      <span class="filename">{jsonFile.name}</span>
      <span class="badge">{entries.length} entrée{entries.length > 1 ? 's' : ''}</span>
    {:else}
      <span class="hint">Déposer un fichier <strong>JSON</strong> ici</span>
    {/if}
  </div>
  <input id="json-input" type="file" accept=".json" hidden onchange={onJsonInput} />

  <div
    class="dropzone"
    class:has={templateFiles.length > 0}
    class:dragover={templateDragOver}
    role="button"
    tabindex="0"
    onclick={() => document.getElementById('template-input')?.click()}
    ondragover={(e) => { e.preventDefault(); templateDragOver = true }}
    ondragleave={() => { templateDragOver = false }}
    ondrop={handleTemplateDrop}
    onkeydown={(e) => { if (e.key === 'Enter') document.getElementById('template-input')?.click() }}
  >
    {#if templateFiles.length > 0}
      <div class="file-list">
        {#each templateFiles as f (f.name)}
          <span class="file-tag">
            {f.name}
            <button class="remove" onclick={(e) => { e.stopPropagation(); removeTemplate(f) }}>×</button>
          </span>
        {/each}
      </div>
    {:else}
      <span class="hint">Déposer les fichiers template <strong>.typ</strong> ici</span>
    {/if}
  </div>
  <input id="template-input" type="file" accept=".typ" multiple hidden onchange={onTemplateInput} />

  <button
    class="generate-btn"
    onclick={generate}
    disabled={processing || entries.length === 0 || templates.size === 0}
  >
    {processing ? 'Génération…' : 'Générer le ZIP'}
  </button>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if progress}
    <div class="progress">{progress}</div>
  {/if}
</div>

<style>
  .app {
    max-width: 640px;
    margin: 3rem auto;
    font-family: system-ui, sans-serif;
  }
  h1 { margin: 0 0 1.5rem; font-size: 1.5rem; }

  .dropzone {
    border: 2px dashed #ccc;
    border-radius: 10px;
    padding: 1.5rem;
    margin-bottom: 1rem;
    text-align: center;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }
  .dropzone:hover { border-color: #888; }
  .dropzone.dragover {
    border-color: #1a1a2e;
    background: #f0f0ff;
  }
  .dropzone.has {
    border-style: solid;
    border-color: #4a4a6a;
    background: #f8f8fc;
  }
  .hint { color: #666; font-size: 0.9rem; }
  .filename { font-weight: 600; }
  .badge {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0.1rem 0.5rem;
    background: #1a1a2e;
    color: white;
    border-radius: 99px;
    font-size: 0.75rem;
  }
  .file-list { display: flex; flex-wrap: wrap; gap: 0.4rem; }
  .file-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.2rem 0.6rem;
    background: #e8e8f0;
    border-radius: 6px;
    font-size: 0.85rem;
  }
  .remove {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 0.1rem;
    color: #991b1b;
  }

  .generate-btn {
    display: block;
    width: 100%;
    padding: 0.75rem;
    font-size: 1rem;
    background: #1a1a2e;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
  }
  .generate-btn:disabled { opacity: 0.4; cursor: default; }

  .progress {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #f0fdf4;
    border: 1px solid #86efac;
    border-radius: 8px;
    color: #166534;
    text-align: center;
  }
  .error {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #fef2f2;
    border: 1px solid #fca5a5;
    border-radius: 8px;
    color: #991b1b;
    font-size: 0.85rem;
  }
</style>
