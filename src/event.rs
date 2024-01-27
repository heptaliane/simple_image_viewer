use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn emit(event: &str, payload: JsValue);
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<T> {
    pub event: String,
    pub id: usize,
    pub payload: T,
    pub window_label: Option<String>,
}
