use gloo::events::EventListener;
use gloo::utils::document;
use shared::config::ViewerConfig;
use shared::event;
use shared::event::{KeyboardEvent, TauriEvent};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use web_sys;

use crate::tauri::{emit_without_args, listen};

fn next_image() {
    emit_without_args(TauriEvent::NextFile.as_ref());
}

fn prev_image() {
    emit_without_args(TauriEvent::PrevFile.as_ref());
}

fn next_directory() {
    emit_without_args(TauriEvent::NextDirectory.as_ref());
}

fn prev_directory() {
    emit_without_args(TauriEvent::PrevDirectory.as_ref())
}

fn handle_keyboard_event(keymap: &HashMap<String, KeyboardEvent>, event: &web_sys::KeyboardEvent) {
    match keymap.get(&event.code()) {
        Some(KeyboardEvent::NextImage) => next_image(),
        Some(KeyboardEvent::PrevImage) => prev_image(),
        Some(KeyboardEvent::NextDirectory) => next_directory(),
        Some(KeyboardEvent::PrevDirectory) => prev_directory(),
        _ => (),
    }
}

pub struct KeyboardEventHandler {
    keymap: Arc<Mutex<HashMap<String, event::KeyboardEvent>>>,
}

impl KeyboardEventHandler {
    pub fn new() -> Self {
        Self {
            keymap: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn listen(&self) {
        emit_without_args(event::TauriEvent::RequestConfig.as_ref());
        {
            let keymap = self.keymap.clone();
            listen(
                event::TauriEvent::ReceiveConfig.as_ref(),
                move |config: ViewerConfig| {
                    let new_keymap: HashMap<String, event::KeyboardEvent> = config
                        .keymap
                        .clone()
                        .into_iter()
                        .flat_map(|(k, vs)| vs.into_iter().map(move |v| (v, k.clone())))
                        .collect();

                    match keymap.lock() {
                        Ok(mut keymap) => *keymap = new_keymap,
                        Err(e) => log::error!("Failed to access keymap: {:?}", e),
                    }
                },
            )
        }

        // KeyEvent listener
        {
            let keymap = self.keymap.clone();
            let kl = EventListener::new(&document(), "keydown", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                match keymap.lock() {
                    Ok(keymap) => handle_keyboard_event(&keymap, event),
                    Err(e) => log::error!("Failed to access keymap: {:?}", e),
                }
            });
            kl.forget();
        }
    }
}
