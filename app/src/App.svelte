<script lang="ts">
  import { onMount } from 'svelte'
  import { BlobWriter, ZipWriter, Uint8ArrayReader, TextReader } from '@zip.js/zip.js'

  let generatePdf: (json: string, template: string) => Uint8Array
  let generateTyp: (json: string, template: string) => string
  let generateCombinedTyp: (templates: string[], jsons: string[]) => string
  let mergePdfs: (pdfs: Uint8Array[]) => Uint8Array

  let jsonFile = $state<File | null>(null)
  let entries = $state<unknown[]>([])
  let templateFiles = $state<File[]>([])
  let templates = $state<Map<string, string>>(new Map())
  let templateWarnings = $state<Record<string, string>>({})
  let processing = $state(false)
  let progress = $state('')
  let error = $state('')
  let jsonDragOver = $state(false)
  let templateDragOver = $state(false)
  let theme = $state<'light' | 'dark'>('light')
  let isDark = $derived(theme === 'dark')

  function applyTheme(t: 'light' | 'dark') {
    theme = t
    document.documentElement.dataset.theme = t
    localStorage.setItem('tambo-theme', t)
  }

  function toggleTheme() {
    applyTheme(theme === 'dark' ? 'light' : 'dark')
  }

  onMount(async () => {
    theme = document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light'
    const mod: any = await import('./wasm/tambo_wasm.js')
    generatePdf = mod.generate_pdf
    generateTyp = mod.generate_standalone_typ
    generateCombinedTyp = mod.generate_combined_typ
    mergePdfs = mod.merge_pdfs
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

    if (!/^\s*#import\s+sys:\s*inputs\b/m.test(content)) {
      templateWarnings[name] =
        'Ce template ne lit pas les données JSON (pas de "#import sys: inputs") : toutes les pages de ce groupe seront identiques.'
    } else {
      delete templateWarnings[name]
    }
  }

  function removeTemplate(file: File) {
    const name = sanitize(file.name.replace(/\.typ$/i, ''))
    templates.delete(name)
    delete templateWarnings[name]
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

  async function compileAll() {
    let pending: { name: string; pdf: Uint8Array; json: string; template: string }[] = []
    let skipped = 0

    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i] as Record<string, unknown>
      const Groupe: unknown = entry.Groupe
      if (!Groupe) { skipped++; continue }

      const templateName = sanitize(String(Groupe))
      const template = templates.get(templateName)
      if (!template) { skipped++; continue }

      progress = `(${i + 1}/${entries.length}) ${Groupe}`
      const jsonStr = JSON.stringify(entry)
      pending.push({
        name: `${String(i + 1).padStart(3, '0')}-${templateName}`,
        pdf: generatePdf(jsonStr, template).slice(),
        json: jsonStr,
        template,
      })
    }

    return { pending, skipped }
  }

  function download(blob: Blob, filename: string) {
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = filename
    a.click()
    URL.revokeObjectURL(url)
  }

  async function generateZip() {
    processing = true
    progress = ''
    error = ''

    const { pending, skipped } = await compileAll()

    if (pending.length === 0) {
      error = 'Aucun fichier généré. Vérifiez que les templates correspondent aux entrées.'
      processing = false
      return
    }

    progress = `Création du ZIP (${pending.length} entrées)...`

    const blobWriter = new BlobWriter('application/zip')
    const zip = new ZipWriter(blobWriter)

    for (const r of pending) {
      await zip.add(`${r.name}.pdf`, new Uint8ArrayReader(r.pdf))
      await zip.add(`${r.name}.typ`, new TextReader(generateTyp(r.json, r.template)))
    }

    const blob = await zip.close()
    download(blob, 'tambo-output.zip')

    progress = `${pending.length} fichier(s) généré(s)${skipped > 0 ? ` (${skipped} ignoré(s))` : ''}`
    processing = false
  }

  async function generateSingle() {
    processing = true
    progress = ''
    error = ''

    const { pending, skipped } = await compileAll()

    if (pending.length === 0) {
      error = 'Aucun fichier généré. Vérifiez que les templates correspondent aux entrées.'
      processing = false
      return
    }

    progress = `Fusion de ${pending.length} PDF(s)...`

    try {
      const merged = mergePdfs(pending.map((r) => r.pdf)).slice()
      download(new Blob([merged], { type: 'application/pdf' }), 'tambo.pdf')
      progress = `PDF unique généré (${pending.length} page(s))${skipped > 0 ? ` (${skipped} ignoré(s))` : ''}`
    } catch (e) {
      error = `Échec de la fusion : ${e}`
    }
    processing = false
  }

  async function generateSingleTyp() {
    processing = true
    progress = ''
    error = ''

    const { pending, skipped } = await compileAll()

    if (pending.length === 0) {
      error = 'Aucun fichier généré. Vérifiez que les templates correspondent aux entrées.'
      processing = false
      return
    }

    progress = `Assemblage de ${pending.length} entrée(s)...`

    try {
      const typ = generateCombinedTyp(pending.map((r) => r.template), pending.map((r) => r.json))
      download(new Blob([typ], { type: 'text/plain' }), 'tambo.typ')
      progress = `.typ unique généré (${pending.length} page(s))${skipped > 0 ? ` (${skipped} ignoré(s))` : ''}`
    } catch (e) {
      error = `Échec de la génération : ${e}`
    }
    processing = false
  }
</script>

<div class="app">
  <div class="header">
    <h1>tambo</h1>
    <button
      class="theme-toggle"
      onclick={toggleTheme}
      aria-label={isDark ? 'Activer le mode clair' : 'Activer le mode sombre'}
      title={isDark ? 'Mode clair' : 'Mode sombre'}
    >
      {isDark ? '☀️' : '🌙'}
    </button>
  </div>

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
      {#if Object.values(templateWarnings).length > 0}
        <div class="warnings">
          {#each Object.entries(templateWarnings) as [name, msg]}
            <div class="warning">⚠ {name} : {msg}</div>
          {/each}
        </div>
      {/if}
    {:else}
      <span class="hint">Déposer les fichiers template <strong>.typ</strong> ici</span>
    {/if}
  </div>
  <input id="template-input" type="file" accept=".typ" multiple hidden onchange={onTemplateInput} />

  <div class="actions">
    <button
      class="generate-btn"
      onclick={generateZip}
      disabled={processing || entries.length === 0 || templates.size === 0}
    >
      {processing ? 'Génération…' : 'Générer le ZIP'}
    </button>
    <button
      class="generate-btn single"
      onclick={generateSingle}
      disabled={processing || entries.length === 0 || templates.size === 0}
    >
      {processing ? 'Génération…' : 'PDF unique'}
    </button>
    <button
      class="generate-btn single"
      onclick={generateSingleTyp}
      disabled={processing || entries.length === 0 || templates.size === 0}
    >
      {processing ? 'Génération…' : '.typ unique'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if progress}
    <div class="progress">{progress}</div>
  {/if}
</div>

<style>
  :global(:root) {
    --bg: #ffffff;
    --text: #1a1a2e;
    --border: #ccc;
    --border-hover: #888;
    --border-strong: #4a4a6a;
    --drop-bg: transparent;
    --drop-drag-bg: #f0f0ff;
    --hint: #666;
    --tag-bg: #e8e8f0;
    --badge-bg: #1a1a2e;
    --btn-bg: #1a1a2e;
    --btn-single-bg: #4a4a6a;
    --btn-text: #ffffff;
    --warning-bg: #fffbeb;
    --warning-border: #fde68a;
    --warning-text: #92400e;
    --progress-bg: #f0fdf4;
    --progress-border: #86efac;
    --progress-text: #166534;
    --error-bg: #fef2f2;
    --error-border: #fca5a5;
    --error-text: #991b1b;
  }

  :global([data-theme="dark"]) {
    --bg: #0f1220;
    --text: #e5e7eb;
    --border: #2d3447;
    --border-hover: #4a5268;
    --border-strong: #3d465c;
    --drop-bg: #141828;
    --drop-drag-bg: #1a1f33;
    --hint: #9aa3b5;
    --tag-bg: #232838;
    --badge-bg: #3d465c;
    --btn-bg: #2a3350;
    --btn-single-bg: #3a4360;
    --btn-text: #f2f4fa;
    --warning-bg: #2e2612;
    --warning-border: #8a6d1f;
    --warning-text: #f0d98c;
    --progress-bg: #0f2418;
    --progress-border: #1f7a45;
    --progress-text: #7ee2a8;
    --error-bg: #2a1216;
    --error-border: #a33737;
    --error-text: #f0a0a0;
  }

  :global(body) {
    background: var(--bg);
    color: var(--text);
    transition: background 0.2s, color 0.2s;
  }

  .app {
    max-width: 640px;
    margin: 3rem auto;
    font-family: system-ui, sans-serif;
    color: var(--text);
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }

  h1 { margin: 0; font-size: 1.5rem; color: #E1344C; }

  .theme-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2.2rem;
    height: 2.2rem;
    font-size: 1.1rem;
    background: var(--btn-single-bg);
    color: var(--btn-text);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }
  .theme-toggle:hover { border-color: var(--border-hover); }

  .dropzone {
    border: 2px dashed var(--border);
    border-radius: 10px;
    padding: 1.5rem;
    margin-bottom: 1rem;
    text-align: center;
    cursor: pointer;
    background: var(--drop-bg);
    transition: background 0.15s, border-color 0.15s;
  }
  .dropzone:hover { border-color: var(--border-hover); }
  .dropzone.dragover {
    border-color: var(--border-strong);
    background: var(--drop-drag-bg);
  }
  .dropzone.has {
    border-style: solid;
    border-color: var(--border-strong);
    background: var(--drop-bg);
  }
  .hint { color: var(--hint); font-size: 0.9rem; }
  .filename { font-weight: 600; }
  .badge {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0.1rem 0.5rem;
    background: var(--badge-bg);
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
    background: var(--tag-bg);
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
    color: var(--error-text);
  }

  .warnings { margin-top: 0.75rem; text-align: left; }
  .warning {
    padding: 0.5rem 0.75rem;
    background: var(--warning-bg);
    border: 1px solid var(--warning-border);
    border-radius: 6px;
    color: var(--warning-text);
    font-size: 0.8rem;
    margin-bottom: 0.4rem;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }
  .generate-btn {
    flex: 1;
    padding: 0.75rem;
    font-size: 1rem;
    background: var(--btn-bg);
    color: var(--btn-text);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
  }
  .generate-btn.single { background: var(--btn-single-bg); }
  .generate-btn:disabled { opacity: 0.4; cursor: default; }

  .progress {
    margin-top: 1rem;
    padding: 0.75rem;
    background: var(--progress-bg);
    border: 1px solid var(--progress-border);
    border-radius: 8px;
    color: var(--progress-text);
    text-align: center;
  }
  .error {
    margin-top: 1rem;
    padding: 0.75rem;
    background: var(--error-bg);
    border: 1px solid var(--error-border);
    border-radius: 8px;
    color: var(--error-text);
    font-size: 0.85rem;
  }
</style>
