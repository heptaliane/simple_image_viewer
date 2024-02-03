use gloo::events::EventListener;
use gloo::utils::document;
use serde_wasm_bindgen::from_value;
use shared::event;
use shared::payload::ImagePayload;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::event::{emit, listen, Event};
use crate::keyboard::handle_keyboard_event;
use crate::tauri::convert_file_src;

#[function_component]
pub fn App() -> Html {
    let path = use_state(|| String::new());

    {
        let path = path.clone();
        use_effect_with_deps(
            move |_| {
                {
                    let path = path.clone();
                    spawn_local(async move {
                        let closure = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                            let event = from_value::<Event<ImagePayload>>(event).unwrap();
                            let uri = convert_file_src(&event.payload.uri, None);
                            path.set(uri.as_string().unwrap());
                        });
                        listen(event::TauriEvent::ReceiveImage.as_ref(), &closure).await;
                        closure.forget();
                    });
                }

                spawn_local(async move {
                    emit(event::TauriEvent::RequestImage.as_ref(), JsValue::NULL).await;
                });

                let listener = EventListener::new(&document(), "keydown", move |e| {
                    handle_keyboard_event(
                        [
                            ("ArrowRight", event::KeyboardEvent::NextImage),
                            ("ArrowLeft", event::KeyboardEvent::PrevImage),
                        ]
                        .iter()
                        .map(|(k, e)| (k.to_string(), e.clone()))
                        .collect(),
                        e.dyn_ref::<web_sys::KeyboardEvent>().unwrap(),
                    );
                });
                listener.forget();
            },
            (),
        );
    }

    html! {
        <img
            src={ (*path).clone() }
        />
    }
}
