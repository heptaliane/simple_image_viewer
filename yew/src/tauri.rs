use js_sys::Promise;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "convertFileSrc")]
    pub fn convert_file_src(path: &str, protocol: Option<&str>) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke")]
    pub fn async_invoke_without_args(cmd: &str) -> Promise;
}
