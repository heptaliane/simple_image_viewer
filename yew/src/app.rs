use serde_wasm_bindgen::from_value;
use shared::event::TauriEvent;
use shared::payload::ImagePayload;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::event::{emit, listen, Event};

#[function_component]
pub fn App() -> Html {
    let path = use_state(|| String::new());

    {
        let path = path.clone();
        spawn_local(async move {
            let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                let event = from_value::<Event<ImagePayload>>(event).unwrap();
                path.set(event.payload.uri);
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
