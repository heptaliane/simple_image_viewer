use serde_wasm_bindgen::from_value;
use shared::payload::FilePayload;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use crate::tauri::{async_invoke_without_args, convert_file_src};

#[function_component]
pub fn App() -> Html {
    let path = use_state(|| JsValue::from_str(""));

    {
        let path = path.clone();
        use_effect_with((), move |_| {
            let path = path.clone();
            spawn_local(async move {
                let promise = async_invoke_without_args("get_file");
                match JsFuture::from(promise).await {
                    Ok(val) => match from_value::<FilePayload>(val) {
                        Ok(fetched) => {
                            path.set(convert_file_src(&fetched.path, None));
                        }
                        Err(e) => {
                            log::error!("Unexpected fetched files format: {:?}", e);
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to fetch files: {:?}", e);
                    }
                }
            })
        })
    }

    html! {
        <img
            src={path.as_string()}
        />
    }
}
