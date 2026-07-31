# AGENTS.md

## Project

CLI tool (`tambo`) that generates PDFs from JSON data using Typst templates. Each JSON entry maps to a template (via the `groupe` field) and produces one PDF. Entries without a `groupe` value are skipped. Each PDF is accompanied by a `.typ` file containing the data embedded inline, compilable standalone.

## Build & Run

```bash
cargo build
cargo run -- -i <json> -t <templates_dir> -o <output_dir>
```

## CLI Flags

- `-i, --input` — JSON file (array of objects)
- `-t, --templates` — Typst `.typ` template directory (default: `templates/`)
- `-o, --output` — PDF output directory (default: `output/`)
- `--field` — JSON field for template selection (default: `groupe`)
- `--root` — root for resolving image paths (default: JSON file's parent dir)

## Architecture (Workspace)

```
tambo/
├── Cargo.toml              (workspace)
├── crates/
│   ├── tambo-core/         (lib — moteur pur)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs    (AppError — thiserror)
│   │   │   ├── json.rs     (json_to_typst_value, json_to_typst_literal)
│   │   │   ├── typst.rs    (compile_entry, compile_entry_simple)
│   │   │   └── generator.rs (sanitize_template_name, generate_standalone_typ)
│   │   └── Cargo.toml
│   └── tambo-wasm/         (cdylib — glue wasm-bindgen)
│       ├── Cargo.toml
│       ├── build.rs        (curl Inter font → OUT_DIR)
│       └── src/lib.rs      (generate_pdf export)
├── src/main.rs             (binaire CLI mince)
├── app/                    (Svelte 5 + Vite, SPA)
│   ├── package.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.ts
│       ├── App.svelte
│       └── wasm/           (output wasm-pack)
└── templates/
```

### Features `tambo-core`

| Feature | Default | Contenu |
|---------|---------|---------|
| `native` | oui | `FileSystemResolver`, `search_fonts_with` (typst-kit-fonts) |
| (aucune) | — | compilation sans filesystem, fonts passées en mémoire |

- `compile_entry` — avec `FileSystemResolver` + `search_fonts_with` (feature `native`)
- `compile_entry_simple` — sans filesystem, prend `&[&[u8]]` pour les fonts (WASM-compatible)

## Templates

- Fichiers `.typ` dans `templates/`, nommés `<sanitized-groupe>.typ`
- Nom dérivé du champ JSON `--field`: lowercase, spaces/underscores → hyphens
- Accèdent aux données via `sys.inputs`:
  ```typst
  #import sys: inputs
  #let d = inputs.data
  ```
- Utiliser `.at("field name")` pour les clés avec espaces/caractères spéciaux
- Chemins d'images résolus depuis `--root`
- `null` JSON → Typst `none`
- Le `.typ` compagnon remplace `#import sys: inputs` par `#let __tambo_data = (...)`

## Template Conventions

Templates receive data via `sys.inputs`:
```typst
#import sys: inputs
#let d = inputs.data
# Title: #d.at("Titre")
```

- Use `.at("field name")` for keys with spaces/special characters
- Images use relative paths resolved from `--root` directory
- `null` JSON values become Typst `none`
- The companion `.typ` file replaces `#import sys: inputs` with `#let __tambo_data = (...)`, so templates must use `inputs.data` to access data (not another name)

## WASM Build

```bash
wasm-pack build crates/tambo-wasm --target web --out-dir ../../app/src/wasm
```

Export unique :

```rust
#[wasm_bindgen]
pub fn generate_pdf(json_str: &str, template: &str) -> Result<Vec<u8>, JsValue>
```

## App (Svelte 5)

```bash
cd app
npm run build:wasm   # wasm-pack build → src/wasm/
npm run dev          # dev server
npm run build        # production build → app/dist/
```

## Key Crates

- `typst-as-lib` 0.16
- `typst-pdf` 0.15
- `typst` 0.15 (utilisé pour `Dict`, `IntoValue`)
- `clap` 4 — CLI argument parsing
- `thiserror` 2 — `AppError`
- `anyhow` 1 — CLI error handling

## Gotchas

- `FileSystemResolver` requis pour les images — les sources statiques seules ne résolvent pas les chemins
- `compile_entry` nécessite la feature `native` (disponible seulement sur le binaire CLI)
- Fonts warnings si polices système absentes
- Le champ `groupe` détermine le template — valeurs `null`/absentes → skip
