use serde_wasm_bindgen::from_value;
use shared::event::TauriEvent;
use shared::payload::ImagePayload;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::event::{emit, listen, Event};
use crate::tauri::convert_file_src;

#[function_component]
pub fn App() -> Html {
    let path = use_state(|| String::new());

    {
        let path = path.clone();
        spawn_local(async move {
            let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                let event = from_value::<Event<ImagePayload>>(event).unwrap();
                let uri = convert_file_src(&event.payload.uri, None);
                path.set(uri.as_string().unwrap());
            });
            listen(TauriEvent::ReceiveImage.as_ref(), &closure).await;
            closure.forget();
        });
    }

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                emit(TauriEvent::RequestImage.as_ref(), JsValue::NULL).await;
            });
        },
        (),
    );

    html! {
        <img
            src={ (*path).clone() }
        />
    }
}
