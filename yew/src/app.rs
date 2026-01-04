use shared::event;
use shared::payload::FilePayload;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::keyboard::KeyboardEventHandler;
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
            let handler = KeyboardEventHandler::new();
            handler.listen();
        });
    }

    html! {
        <img
            src={path.as_string()}
        />
    }
}
