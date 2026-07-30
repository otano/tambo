use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_pdf(json_str: &str, template: &str) -> std::result::Result<Vec<u8>, JsValue> {
    let entry: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from(js_sys::Error::new(&format!("JSON invalide : {e}"))))?;

    let font_bytes: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/Inter-Regular.ttf"));

    tambo_core::compile_entry_simple(&entry, template, &[font_bytes])
        .map_err(|e| JsValue::from(js_sys::Error::new(&e.to_string())))
}
