//! Tests d'intégration : compilation des vrais templates du repo avec des
//! entrées au schéma actuel. Nécessite la feature `native` (fonts système).

#![cfg(feature = "native")]

use std::path::PathBuf;

use serde_json::Value;

fn templates_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../templates")
}

fn load_template(name: &str) -> String {
    std::fs::read_to_string(templates_dir().join(name))
        .unwrap_or_else(|_| panic!("template manquant : {name}"))
}

fn entry(groupe: &str, titre: &str, auteur: &str) -> Value {
    serde_json::json!({
        "Auteur": auteur,
        "Titre": titre,
        "Date": "1810",
        "Credit": null,
        "Inventaire": "G.5224",
        "Explicatif": format!("Description de {titre}"),
        "Traduction": format!("English description of {titre}"),
        "Technique": "estampe",
        "Preteur": "Musée Carnavalet",
        "Groupe": groupe,
    })
}

#[test]
fn real_templates_compile_distinct_entries() {
    let simple = load_template("cartel-simple.typ");
    let develop = load_template("cartel-develop.typ");
    let root = templates_dir().join("../..");

    let e1 = entry("cartel-simple", "Feu d'artifice à l'Arc de triomphe", "Debucourt");
    let e2 = entry("cartel-simple", "L'Arc de triomphe la nuit", "Schall");
    let e3 = entry("cartel-develop", "Le cénotaphe sous l'Arc de triomphe", "Cuville");

    let p1 = tambo_core::compile_entry(&e1, &simple, &root).unwrap();
    let p2 = tambo_core::compile_entry(&e2, &simple, &root).unwrap();
    let p3 = tambo_core::compile_entry(&e3, &develop, &root).unwrap();

    for p in [&p1, &p2, &p3] {
        assert!(p.starts_with(b"%PDF-"), "PDF attendu, obtenu un autre format");
        assert!(!p.is_empty());
    }

    assert_ne!(
        p1, p2,
        "deux entrées cartel-simple distinctes doivent produire des PDF différents"
    );
    assert_ne!(
        p1, p3,
        "une entrée cartel-simple et une cartel-develop doivent produire des PDF différents"
    );
}

#[test]
fn real_templates_combined_typ_produces_distinct_pages() {
    let simple = load_template("cartel-simple.typ");
    let develop = load_template("cartel-develop.typ");

    let e1 = entry("cartel-simple", "Feu d'artifice à l'Arc de triomphe", "Debucourt");
    let e2 = entry("cartel-develop", "Le cénotaphe sous l'Arc de triomphe", "Cuville");

    let items: Vec<(&str, &Value)> = vec![(simple.as_str(), &e1), (develop.as_str(), &e2)];
    let combined = tambo_core::generate_combined_typ(&items);

    let pdf = tambo_core::compile_entry_simple(&serde_json::json!({}), &combined, &[]).unwrap();
    let doc = lopdf::Document::load_mem(&pdf).unwrap();
    assert_eq!(doc.get_pages().len(), 2, "le .typ combiné doit produire 2 pages");

    let contents: Vec<Vec<u8>> = doc
        .get_pages()
        .values()
        .map(|page_id| {
            let mut bytes = Vec::new();
            for id in doc.get_page_contents(*page_id) {
                if let Ok(lopdf::Object::Stream(stream)) = doc.get_object(id) {
                    if let Ok(c) = stream.decompressed_content() {
                        bytes.extend_from_slice(&c);
                    }
                }
            }
            bytes
        })
        .collect();
    assert_eq!(contents.len(), 2);
    assert_ne!(contents[0], contents[1], "les 2 pages du .typ combiné doivent différer");
}
