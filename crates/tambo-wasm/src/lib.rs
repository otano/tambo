use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn generate_pdf(json_str: &str, template: &str) -> std::result::Result<Vec<u8>, JsValue> {
    let entry: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from(js_sys::Error::new(&format!("JSON invalide : {e}"))))?;

    let font_bytes: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/Inter-Regular.ttf"));

    tambo_core::compile_entry_simple(&entry, template, &[font_bytes])
        .map_err(|e| JsValue::from(js_sys::Error::new(&e.to_string())))
}

#[wasm_bindgen]
pub fn generate_standalone_typ(json_str: &str, template: &str) -> std::result::Result<String, JsValue> {
    let entry: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from(js_sys::Error::new(&format!("JSON invalide : {e}"))))?;

    Ok(tambo_core::generate_standalone_typ(template, &entry))
}

#[wasm_bindgen]
pub fn merge_pdfs(pdfs: &Array) -> std::result::Result<Vec<u8>, JsValue> {
    let mut items: Vec<Vec<u8>> = Vec::new();
    for i in 0..pdfs.length() {
        let bytes = pdfs
            .get(i)
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from(js_sys::Error::new("Élément PDF invalide")))?;
        items.push(bytes.to_vec());
    }
    let refs: Vec<&[u8]> = items.iter().map(|b| b.as_slice()).collect();
    tambo_core::merge_pdfs(&refs)
        .map_err(|e| JsValue::from(js_sys::Error::new(&e.to_string())))
}
