use shared::event::{KeyboardEvent, TauriEvent};
use std::collections::HashMap;
use web_sys;

use crate::tauri::emit_without_args;

fn next_image() {
    emit_without_args(TauriEvent::NextFile.as_ref());
}

fn prev_image() {
    emit_without_args(TauriEvent::PrevFile.as_ref());
}

pub fn handle_keyboard_event(
    keymap: HashMap<String, KeyboardEvent>,
    event: &web_sys::KeyboardEvent,
) {
    match keymap.get(&event.code()) {
        Some(KeyboardEvent::NextImage) => next_image(),
        Some(KeyboardEvent::PrevImage) => prev_image(),
        _ => (),
    }
}
