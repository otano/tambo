use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
#[command(name = "tambo", about = "Generate PDFs from JSON data using Typst templates")]
struct Cli {
    /// Input JSON file (array of objects)
    #[arg(short, long)]
    input: PathBuf,

    /// Directory containing Typst templates (.typ files)
    #[arg(short, long, default_value = "templates")]
    templates: PathBuf,

    /// Output directory for generated PDFs
    #[arg(short, long, default_value = "output")]
    output: PathBuf,

    /// JSON field used to select the template (e.g. "Groupe")
    #[arg(long, default_value = "Groupe")]
    field: String,

    /// Root directory for resolving relative image paths (defaults to JSON file's parent)
    #[arg(long)]
    root: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        bail!("Input file not found: {}", cli.input.display());
    }
    if !cli.templates.exists() {
        bail!("Templates directory not found: {}", cli.templates.display());
    }

    let raw = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read {}", cli.input.display()))?;
    let entries: Vec<Value> =
        serde_json::from_str(&raw).with_context(|| "JSON must be an array of objects")?;

    fs::create_dir_all(&cli.output)?;

    let root = cli
        .root
        .map(|p| fs::canonicalize(&p).unwrap_or(p))
        .unwrap_or_else(|| {
            cli.input
                .canonicalize()
                .ok()
                .and_then(|p| p.parent().map(|pp| pp.to_path_buf()))
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        });

    let mut template_cache: HashMap<String, String> = HashMap::new();

    for (i, entry) in entries.iter().enumerate() {
        let template_name = match entry.get(&cli.field).and_then(|v| v.as_str()) {
            Some(name) => tambo_core::sanitize_template_name(name),
            None => {
                eprintln!(
                    "[{}/{}] SKIP: no `{}` field",
                    i + 1,
                    entries.len(),
                    cli.field,
                );
                continue;
            }
        };

        let template_source = match template_cache.get(&template_name) {
            Some(src) => src.clone(),
            None => {
                let path = cli.templates.join(format!("{template_name}.typ"));
                if !path.exists() {
                    eprintln!(
                        "[{}/{}] SKIP: template not found: {}",
                        i + 1,
                        entries.len(),
                        path.display()
                    );
                    continue;
                }
                let src = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read {}", path.display()))?;
                template_cache.insert(template_name.clone(), src.clone());
                src
            }
        };

        let pdf_name = match entry.get("DEXID").and_then(|v| v.as_str()) {
            Some(name) => name.to_string(),
            None => i.to_string(),
        };

        let pdf_path = cli.output.join(format!("{pdf_name}.pdf"));

        eprint!("[{}/{}] {} ... ", i + 1, entries.len(), pdf_path.display());

        match tambo_core::compile_entry(entry, &template_source, &root) {
            Ok(pdf_bytes) => {
                fs::write(&pdf_path, &pdf_bytes)?;
                let typ_path = pdf_path.with_extension("typ");
                let typ_source = tambo_core::generate_standalone_typ(&template_source, entry);
                fs::write(&typ_path, &typ_source)?;
                eprintln!("ok ({} bytes, typ)", pdf_bytes.len());
            }
            Err(e) => {
                eprintln!("FAILED: {e}");
            }
        }
    }

    Ok(())
}
