use js_sys::Promise;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "convertFileSrc")]
    pub fn convert_file_src(path: &str, protocol: Option<&str>) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke")]
    pub fn async_invoke_without_args(cmd: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = "listen")]
    fn tauri_listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = "emit")]
    fn tauri_emit(event: &str, payload: &JsValue) -> Promise;
}

#[derive(Deserialize)]
struct ListenEvent<T> {
    payload: T,
}

pub fn listen<F, T>(event: &str, mut handler: F)
where
    F: FnMut(T) + 'static,
    T: DeserializeOwned,
{
    let closure = Closure::wrap(Box::new({
        let event = event.to_string();
        move |val: JsValue| match from_value::<ListenEvent<T>>(val) {
            Ok(v) => handler(v.payload),
            Err(e) => log::error!("[listen] ({:?}) {:?}", event, e),
        }
    }) as Box<dyn FnMut(JsValue)>);
    tauri_listen(event, &closure);
    closure.forget();
}

pub fn emit_without_args(event: &str) {
    let event = event.to_string();
    spawn_local(async move {
        match JsFuture::from(tauri_emit(&event, &JsValue::NULL)).await {
            Err(e) => log::error!("[emit] ({:?}) {:?}", event, e),
            _ => log::info!("completed"),
        }
    })
}
