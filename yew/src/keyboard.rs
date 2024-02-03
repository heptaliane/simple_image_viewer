use shared::event;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys;

use crate::event::emit;

fn next_image() {
    spawn_local(async move {
        emit(event::TauriEvent::MoveNext.as_ref(), JsValue::NULL).await;
    });
}

fn prev_image() {
    spawn_local(async move {
        emit(event::TauriEvent::MovePrev.as_ref(), JsValue::NULL).await;
    });
}

pub fn handle_keyboard_event(keymap: HashMap<String, event::KeyboardEvent>, event: &web_sys::KeyboardEvent) {
    match keymap.get(&event.code()) {
        Some(event::KeyboardEvent::NextImage) => next_image(),
        Some(event::KeyboardEvent::PrevImage) => prev_image(),
        _ => (),
    }
}
