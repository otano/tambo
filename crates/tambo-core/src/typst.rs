use std::path::Path;

use serde_json::Value;
use typst::foundations::Dict;

#[cfg(feature = "native")]
use typst_as_lib::file_resolver::FileSystemResolver;
#[cfg(feature = "native")]
use typst_as_lib::typst_kit_options::TypstKitFontOptions;

use crate::error::Result;
use crate::json::json_to_typst_value;

fn build_engine(
    template_source: &str,
    fonts: Option<&[&[u8]]>,
    #[allow(unused_variables)] root: Option<&Path>,
) -> typst_as_lib::TypstEngine {
    let main_file_id = "main.typ";
    let sources: Vec<(&str, &str)> = vec![(main_file_id, template_source)];

    let mut builder = typst_as_lib::TypstEngine::builder()
        .with_static_source_file_resolver(sources);

    #[cfg(feature = "native")]
    if let Some(root) = root {
        builder = builder.add_file_resolver(FileSystemResolver::new(root.to_path_buf()));
    }

    #[cfg(feature = "native")]
    {
        builder = builder.search_fonts_with(TypstKitFontOptions::default());
    }

    if let Some(fonts) = fonts {
        if !fonts.is_empty() {
            builder = builder.fonts(fonts.iter().copied());
        }
    }

    builder.build()
}

/// Compile a single entry with filesystem support (native only).
#[cfg(feature = "native")]
pub fn compile_entry(
    entry: &Value,
    template_source: &str,
    root: &Path,
) -> Result<Vec<u8>> {
    let engine = build_engine(template_source, None, Some(root));
    compile_with_engine(entry, &engine)
}

/// Compile a single entry without filesystem, with custom font bytes.
pub fn compile_entry_simple(
    entry: &Value,
    template_source: &str,
    fonts: &[&[u8]],
) -> Result<Vec<u8>> {
    let engine = build_engine(template_source, Some(fonts), None);
    compile_with_engine(entry, &engine)
}

fn compile_with_engine(entry: &Value, engine: &typst_as_lib::TypstEngine) -> Result<Vec<u8>> {
    let mut inputs = Dict::new();
    inputs.insert(
        "data".into(),
        json_to_typst_value(entry),
    );

    let warned = engine.compile_with_input("main.typ", inputs);

    for w in &warned.warnings {
        eprintln!("  warning: {w:?}");
    }

    let doc = warned
        .output
        .map_err(|e| crate::error::AppError::TypstCompilation(format!("{e:?}")))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| crate::error::AppError::TypstCompilation(format!("PDF export failed: {e:?}")))?;

    Ok(pdf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_entry_simple_twice() {
        let entry = json!({"Titre": "A", "Auteur": "Test1"});
        let template = r#"#import sys: inputs
#let d = inputs.data
#d.at("Titre")
"#;
        let r1 = compile_entry_simple(&entry, template, &[]).unwrap();
        let entry2 = json!({"Titre": "B", "Auteur": "Test2"});
        let r2 = compile_entry_simple(&entry2, template, &[]).unwrap();
        assert!(r1.starts_with(b"%PDF-"));
        assert!(r2.starts_with(b"%PDF-"));
        assert_ne!(r1, r2, "deux appels doivent produire des PDF différents");
    }

    #[test]
    fn test_compile_entry_simple_minimal() {
        let entry = json!({
            "Titre": "Test",
            "Auteur": "Test",
            "Date": "2024",
            "Explicatif": null,
            "Credit": "CC0",
            "Traduction": null,
            "Pays": "France",
            "Ville": "Paris",
            "Domaine": "Test",
            "Technique": "test",
            "Preteur": "Test",
            "Inventaire": "T-01",
            "DEXID": "01",
            "Image": null,
        });

        let template = r#"
#import sys: inputs
#let d = inputs.data
#d.at("Titre")
"#;

        let result = compile_entry_simple(&entry, template, &[]);
        assert!(result.is_ok(), "compilation should succeed: {:?}", result.err());
        let pdf = result.unwrap();
        assert!(!pdf.is_empty(), "PDF should not be empty");
        assert!(pdf.starts_with(b"%PDF-"), "should start with PDF magic bytes");
    }
}
