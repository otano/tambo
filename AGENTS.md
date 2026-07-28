# AGENTS.md

## Project

CLI tool (`tambo`) that generates PDFs from JSON data using Typst templates. Each JSON entry maps to a template (via the `Section` field) and produces one PDF.

## Build & Run

```bash
cargo build
cargo run -- -i <json> -t <templates_dir> -o <output_dir>
```

## CLI Flags

- `-i, --input` — JSON file (array of objects)
- `-t, --templates` — Typst `.typ` template directory (default: `templates/`)
- `-o, --output` — PDF output directory (default: `output/`)
- `--field` — JSON field for template selection (default: `Section`)
- `--default-template` — fallback template name (default: `default`)
- `--root` — root for resolving image paths (default: JSON file's parent dir)

## Architecture

- `src/main.rs` — single-file CLI, uses `typst-as-lib` for compilation
- Templates are `.typ` files in `templates/`, named `<sanitized-section>.typ`
- Template name derived from JSON `--field`: lowercase, spaces → hyphens
- Data injected via `sys.inputs` — templates access it with `#import sys: inputs`

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

## Key Crates

- `typst-as-lib` 0.16 — compiles Typst from Rust, uses `with_static_source_file_resolver` + `FileSystemResolver`
- `typst-pdf` 0.15 — exports compiled document to PDF bytes
- `typ` 0.15 — underlying Typst compiler (used for `Dict`, `IntoValue`)
- `clap` 4 — CLI argument parsing

## Gotchas

- `FileSystemResolver` is required for image loading — detached sources alone don't resolve filesystem paths
- Font warnings are expected if system fonts aren't installed (templates use `Linux Libertine` by default)
- The `Section` field value determines template selection — entries with missing/unmatched sections use `--default-template`
