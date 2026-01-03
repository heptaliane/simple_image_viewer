use gloo::events::EventListener;
use gloo::utils::document;
use shared::event;
use shared::payload::FilePayload;
use wasm_bindgen::{JsCast, JsValue};
use web_sys;
use yew::prelude::*;

use crate::keyboard::handle_keyboard_event;
use crate::tauri::{convert_file_src, emit_without_args, listen};

#[function_component]
pub fn App() -> Html {
    let path = use_state(|| JsValue::from_str(""));

    {
        use_effect_with((), move |_| {
            emit_without_args(event::TauriEvent::RequestFile.as_ref());
        });
    }
    {
        let path = path.clone();
        use_effect_with((), move |_| {
            listen(
                event::TauriEvent::ReceiveFile.as_ref(),
                move |p: FilePayload| {
                    path.set(convert_file_src(&p.path, None));
                },
            );
        });
    }
    {
        use_effect_with((), move |_| {
            let listener = EventListener::new(&document(), "keydown", move |event| {
                handle_keyboard_event(
                    [
                        ("ArrowRight", event::KeyboardEvent::NextImage),
                        ("ArrowLeft", event::KeyboardEvent::PrevImage),
                        ("ArrowDown", event::KeyboardEvent::NextDirectory),
                        ("ArrowUp", event::KeyboardEvent::PrevDirectory),
                    ]
                    .iter()
                    .map(|(k, e)| (k.to_string(), e.clone()))
                    .collect(),
                    event.dyn_ref::<web_sys::KeyboardEvent>().unwrap(),
                );
            });
            listener.forget();
        });
    }

    html! {
        <img
            src={path.as_string()}
        />
    }
}
