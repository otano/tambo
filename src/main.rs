use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde_json::Value;
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::TypstEngine;
use typst_as_lib::file_resolver::FileSystemResolver;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;

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

    /// JSON field used to select the template (e.g. "Section")
    #[arg(long, default_value = "Section")]
    field: String,

    /// Fallback template name (without .typ) when field is missing or template not found
    #[arg(long, default_value = "default")]
    default_template: String,

    /// Root directory for resolving relative image paths (defaults to JSON file's parent)
    #[arg(long)]
    root: Option<PathBuf>,
}

fn sanitize_template_name(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .replace('_', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

fn json_to_typst_value(val: &Value) -> typst::foundations::Value {
    match val {
        Value::Null => typst::foundations::Value::None,
        Value::Bool(b) => (*b).into_value(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_value()
            } else if let Some(f) = n.as_f64() {
                f.into_value()
            } else {
                typst::foundations::Value::None
            }
        }
        Value::String(s) => s.as_str().into_value(),
        Value::Array(arr) => {
            let items: Vec<typst::foundations::Value> =
                arr.iter().map(json_to_typst_value).collect();
            items.into_value()
        }
        Value::Object(map) => {
            let mut dict = Dict::new();
            for (k, v) in map {
                dict.insert(k.as_str().into(), json_to_typst_value(v));
            }
            dict.into_value()
        }
    }
}

fn compile_entry(
    entry: &Value,
    template_source: &str,
    root: &Path,
) -> Result<Vec<u8>> {
    let mut inputs = Dict::new();
    inputs.insert("data".into(), json_to_typst_value(entry));

    let main_file_id = "main.typ";
    let sources: Vec<(&str, &str)> = vec![(main_file_id, template_source)];

    let engine = TypstEngine::builder()
        .with_static_source_file_resolver(sources)
        .add_file_resolver(FileSystemResolver::new(root.to_path_buf()))
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    let warned = engine.compile_with_input(main_file_id, inputs);

    if !warned.warnings.is_empty() {
        for w in &warned.warnings {
            eprintln!("  warning: {w:?}");
        }
    }

    let doc = warned.output.map_err(|e| anyhow::anyhow!("Typst compilation failed: {e:?}"))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| anyhow::anyhow!("PDF export failed: {e:?}"))?;

    Ok(pdf)
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
        let template_name = entry
            .get(&cli.field)
            .and_then(|v| v.as_str())
            .map(sanitize_template_name)
            .unwrap_or_else(|| cli.default_template.clone());

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

        match compile_entry(entry, &template_source, &root) {
            Ok(pdf_bytes) => {
                fs::write(&pdf_path, &pdf_bytes)?;
                eprintln!("ok ({} bytes)", pdf_bytes.len());
            }
            Err(e) => {
                eprintln!("FAILED: {e}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sanitize_template_name_lowercase() {
        assert_eq!(sanitize_template_name("Section 1"), "section-1");
    }

    #[test]
    fn test_sanitize_template_name_underscore_to_hyphen() {
        assert_eq!(sanitize_template_name("my_section"), "my-section");
    }

    #[test]
    fn test_sanitize_template_name_strip_special_chars() {
        assert_eq!(sanitize_template_name("Section 1: Intro!"), "section-1-intro");
    }

    #[test]
    fn test_sanitize_template_name_trim() {
        assert_eq!(sanitize_template_name("  Default  "), "default");
    }

    #[test]
    fn test_json_to_typst_value_string() {
        let v = json_to_typst_value(&json!("hello"));
        let s = format!("{:?}", v);
        assert!(s.contains("hello"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_null() {
        let v = json_to_typst_value(&Value::Null);
        assert_eq!(format!("{:?}", v), "None");
    }

    #[test]
    fn test_json_to_typst_value_integer() {
        let v = json_to_typst_value(&json!(42));
        let s = format!("{:?}", v);
        assert!(s.contains("42"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_bool() {
        let v = json_to_typst_value(&json!(true));
        let s = format!("{:?}", v);
        assert!(s.contains("true"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_array() {
        let v = json_to_typst_value(&json!(["a", "b"]));
        let s = format!("{:?}", v);
        assert!(s.contains('a'), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_object() {
        let v = json_to_typst_value(&json!({"key": "val"}));
        let s = format!("{:?}", v);
        assert!(s.contains("val"), "got {s:?}");
    }

    #[test]
    fn test_compile_entry_minimal() {
        let entry = json!({
            "Titre": "Test",
            "Auteur": "Test",
            "Date": "2024",
            "explicatif": null,
            "Credit line": "CC0",
            "traduction": null,
            "Pays": "France",
            "Ville": "Paris",
            "Domaine": "Test",
            "Designation rédigee": "test",
            "Prêteur": "Test",
            "N° inventaire prêteur": "T-01",
            "DEXID": "01",
            "Image ref": null,
        });

        let template = r#"
#import sys: inputs
#let d = inputs.data
#d.at("Titre")
"#;

        let result = compile_entry(&entry, template, Path::new("."));
        assert!(result.is_ok(), "compilation should succeed: {:?}", result.err());
        let pdf = result.unwrap();
        assert!(!pdf.is_empty(), "PDF should not be empty");
        assert!(pdf.starts_with(b"%PDF-"), "should start with PDF magic bytes");
    }
}
