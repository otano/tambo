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

## Architecture

- `src/main.rs` — single-file CLI, uses `typst-as-lib` for compilation
- Templates are `.typ` files in `templates/`, named `<sanitized-groupe>.typ`
- Template name derived from JSON `--field`: lowercase, spaces/underscores → hyphens
- Data injected via `sys.inputs` — templates access it with `#import sys: inputs`
- Each PDF has a companion `.typ` file with data embedded inline (`__tambo_data`), compilable standalone

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

## Key Crates

- `typst-as-lib` 0.16 — compiles Typst from Rust, uses `with_static_source_file_resolver` + `FileSystemResolver`
- `typst-pdf` 0.15 — exports compiled document to PDF bytes
- `typ` 0.15 — underlying Typst compiler (used for `Dict`, `IntoValue`)
- `clap` 4 — CLI argument parsing

## Gotchas

- `FileSystemResolver` is required for image loading — detached sources alone don't resolve filesystem paths
- Font warnings are expected if system fonts aren't installed (templates use `Linux Libertine` by default)
- The `groupe` field value determines template selection — entries with missing/null `groupe` are skipped
