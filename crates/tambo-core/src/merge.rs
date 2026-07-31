use std::collections::HashMap;

use lopdf::{Dictionary, Document, Object, ObjectId};

use crate::error::{AppError, Result};

/// Merge several PDFs into a single PDF, pages concatenated in order.
pub fn merge_pdfs(pdfs: &[&[u8]]) -> Result<Vec<u8>> {
    if pdfs.is_empty() {
        return Err(AppError::InvalidInput("Aucun PDF à fusionner".to_string()));
    }
    if pdfs.len() == 1 {
        return Ok(pdfs[0].to_vec());
    }

    let mut merged = Document::with_version("1.7");
    let mut next_free: u32 = merged.max_id;
    let mut page_tree_ids: Vec<ObjectId> = Vec::new();
    let mut page_count = 0u32;

    for pdf in pdfs {
        let doc = Document::load_mem(pdf)
            .map_err(|e| AppError::InvalidInput(format!("Lecture du PDF : {e}")))?;

        page_count += doc.get_pages().len() as u32;

        let tree_root = page_tree_root(&doc)?;
        let delta = next_free + 1;

        let mut map: HashMap<ObjectId, ObjectId> = HashMap::new();
        for &id in doc.objects.keys() {
            map.insert(id, (id.0 + delta, id.1));
        }

        for (id, obj) in &doc.objects {
            let mut copied = obj.clone();
            remap_references(&mut copied, &map);
            let new_id = map[&id];
            merged.objects.insert(new_id, copied);
            next_free = next_free.max(new_id.0);
        }

        page_tree_ids.push(map[&tree_root]);
    }

    let tree_id = (next_free + 1, 0);
    let mut tree = Dictionary::new();
    tree.set("Type", Object::Name(b"Pages".to_vec()));
    tree.set(
        "Kids",
        Object::Array(
            page_tree_ids
                .iter()
                .map(|&id| Object::Reference(id))
                .collect(),
        ),
    );
    tree.set("Count", Object::Integer(page_count as i64));
    merged.objects.insert(tree_id, Object::Dictionary(tree));

    for &id in &page_tree_ids {
        let obj = merged
            .objects
            .get_mut(&id)
            .ok_or_else(|| AppError::InvalidInput("Arbre de pages introuvable".to_string()))?;
        if let Object::Dictionary(d) = obj {
            d.set("Parent", Object::Reference(tree_id));
        }
    }

    let catalog_id = (tree_id.0 + 1, 0);
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(tree_id));
    merged
        .objects
        .insert(catalog_id, Object::Dictionary(catalog));
    merged.trailer.set("Root", Object::Reference(catalog_id));
    merged.max_id = catalog_id.0;

    let mut out = Vec::new();
    merged
        .save_to(&mut out)
        .map_err(|e| AppError::InvalidInput(format!("Sauvegarde du PDF fusionné : {e}")))?;
    Ok(out)
}

fn page_tree_root(doc: &Document) -> Result<ObjectId> {
    let root = doc
        .trailer
        .get(b"Root")
        .ok()
        .and_then(|o| o.as_reference().ok())
        .ok_or_else(|| AppError::InvalidInput("PDF sans catalogue".to_string()))?;
    let catalog = doc
        .get_object(root)
        .ok()
        .and_then(|o| o.as_dict().ok())
        .ok_or_else(|| AppError::InvalidInput("Catalogue invalide".to_string()))?;
    catalog
        .get(b"Pages")
        .ok()
        .and_then(|o| o.as_reference().ok())
        .ok_or_else(|| AppError::InvalidInput("PDF sans arbre de pages".to_string()))
}

fn remap_references(obj: &mut Object, map: &HashMap<ObjectId, ObjectId>) {
    match obj {
        Object::Reference(id) => {
            if let Some(new) = map.get(id) {
                *id = *new;
            }
        }
        Object::Array(items) => {
            for item in items {
                remap_references(item, map);
            }
        }
        Object::Dictionary(dict) => {
            for (_, v) in dict.iter_mut() {
                remap_references(v, map);
            }
        }
        Object::Stream(stream) => {
            for (_, v) in stream.dict.iter_mut() {
                remap_references(v, map);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst::compile_entry_simple;
    use serde_json::json;

    fn sample_pdf(title: &str) -> Vec<u8> {
        let template = "#import sys: inputs\n#let d = inputs.data\n#d.at(\"Titre\")";
        let entry = json!({ "Titre": title });
        compile_entry_simple(&entry, template, &[]).unwrap()
    }

    fn page_contents(doc: &Document) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for (_num, page_id) in doc.get_pages() {
            let mut bytes = Vec::new();
            for id in doc.get_page_contents(page_id) {
                if let Ok(Object::Stream(stream)) = doc.get_object(id) {
                    if let Ok(c) = stream.decompressed_content() {
                        bytes.extend_from_slice(&c);
                    }
                }
            }
            out.push(bytes);
        }
        out
    }

    #[test]
    fn test_merge_pdfs_concatenates_pages() {
        let a = sample_pdf("Page A");
        let b = sample_pdf("Page B");
        let c = sample_pdf("Page C");

        let merged = merge_pdfs(&[&a, &b, &c]).unwrap();
        assert!(merged.starts_with(b"%PDF-"));
        assert!(merged.len() > a.len());

        let doc = Document::load_mem(&merged).unwrap();
        assert_eq!(doc.get_pages().len(), 3, "le PDF fusionné doit avoir 3 pages");
    }

    #[test]
    fn test_merge_pdfs_preserves_distinct_content() {
        let a = sample_pdf("Cuville");
        let b = sample_pdf("Schall");
        let c = sample_pdf("Debucourt");

        let merged = merge_pdfs(&[&a, &b, &c]).unwrap();
        let doc = Document::load_mem(&merged).unwrap();
        assert_eq!(doc.get_pages().len(), 3);

        let contents = page_contents(&doc);
        assert_eq!(contents.len(), 3, "chaque page doit avoir un flux de contenu");
        assert_ne!(
            contents[0], contents[1],
            "les pages 1 et 2 doivent avoir un contenu différent"
        );
        assert_ne!(
            contents[1], contents[2],
            "les pages 2 et 3 doivent avoir un contenu différent"
        );
        assert_ne!(
            contents[0], contents[2],
            "les pages 1 et 3 doivent avoir un contenu différent"
        );
    }

    #[test]
    fn test_merge_single_pdf_identity() {
        let a = sample_pdf("Solo");
        let merged = merge_pdfs(&[&a]).unwrap();
        assert_eq!(merged, a);
    }

    #[test]
    fn test_merge_empty_fails() {
        assert!(merge_pdfs(&[]).is_err());
    }
}
