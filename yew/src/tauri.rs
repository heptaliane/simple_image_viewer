use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "convertFileSrc")]
    pub fn convert_file_src(path: &str, protocol: Option<&str>) -> JsValue;
}
