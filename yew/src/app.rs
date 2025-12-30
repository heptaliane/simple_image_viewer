use serde_wasm_bindgen::from_value;
use shared::payload::FilePathPayload;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use crate::tauri::{async_invoke_without_args, convert_file_src};

#[function_component]
pub fn App() -> Html {
    let paths = use_state(|| vec![]);
    let cursor = use_state(|| 0);

    {
        let paths = paths.clone();
        use_effect_with((), move |_| {
            let paths = paths.clone();
            spawn_local(async move {
                let promise = async_invoke_without_args("get_files");
                match JsFuture::from(promise).await {
                    Ok(val) => match from_value::<FilePathPayload>(val) {
                        Ok(fetched) => {
                            paths.set(fetched.paths);
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
            src={
                if *cursor < paths.len() {
                    convert_file_src(&paths[*cursor], None).as_string().unwrap()
                } else {
                    String::new()
                }
            }
        />
    }
}
