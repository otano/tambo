<script lang="ts">
  let jsonStr = $state(`{
  "Titre": "Feu d'artifice à l'Arc de triomphe",
  "Auteur": "Philibert Louis Debucourt",
  "Date": "1810",
  "explicatif": "Le 2 avril 1810, Napoléon 1er épouse Marie-Louise d'Autriche.",
  "Credit line": "CC0 Paris Musées"
}`)

  let templateStr = $state(`#import sys: inputs
#let d = inputs.data
#set page(width: 10cm, height: auto, margin: 1.5cm)
#set text(font: "Inter", size: 10pt)

#align(center)[
  #text(size: 14pt, weight: "bold")[#d.at("Titre")]
  #v(0.3cm)
  #text(size: 11pt)[#d.at("Auteur")]
  #v(0.2cm)
  #text(size: 9pt, fill: gray)[#d.at("Date")]
]

#if d.at("explicatif") != none {
  v(0.5cm)
  set par(justify: true)
  text(size: 9pt)[#d.at("explicatif")]
}`)

  let loading = $state(false)
  let error = $state('')

  async function compile() {
    loading = true
    error = ''

    try {
      const { default: init, generate_pdf } = await import('./wasm/tambo_wasm.js')
      await init()
      const pdf = generate_pdf(jsonStr, templateStr)

      const blob = new Blob([pdf], { type: 'application/pdf' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'document.pdf'
      a.click()
      URL.revokeObjectURL(url)
    } catch (e) {
      error = String(e)
    } finally {
      loading = false
    }
  }
</script>

<div class="app">
  <h1>tambo</h1>
  <p>Générateur de PDF depuis JSON + template Typst</p>

  <div class="panels">
    <div class="panel">
      <h2>Données JSON</h2>
      <textarea bind:value={jsonStr} rows={12} spellcheck="false"></textarea>
    </div>

    <div class="panel">
      <h2>Template Typst</h2>
      <textarea bind:value={templateStr} rows={12} spellcheck="false"></textarea>
    </div>
  </div>

  <button onclick={compile} disabled={loading}>
    {loading ? 'Compilation…' : 'Générer le PDF'}
  </button>

  {#if error}
    <div class="error">{error}</div>
  {/if}
</div>

<style>
  .app {
    max-width: 900px;
    margin: 2rem auto;
    font-family: system-ui, sans-serif;
  }
  .panels {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    margin: 1rem 0;
  }
  .panel h2 {
    margin: 0 0 0.5rem;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #666;
  }
  textarea {
    width: 100%;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 0.8rem;
    border: 1px solid #ddd;
    border-radius: 6px;
    padding: 0.5rem;
    resize: vertical;
    box-sizing: border-box;
  }
  button {
    padding: 0.6rem 1.5rem;
    font-size: 1rem;
    background: #1a1a2e;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.5;
  }
  .error {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #fef2f2;
    border: 1px solid #fca5a5;
    border-radius: 6px;
    color: #991b1b;
    font-size: 0.85rem;
    white-space: pre-wrap;
  }
</style>
