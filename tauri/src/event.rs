use std::sync::{Arc, Mutex};

use tauri::{self, Manager, Emitter};

use crate::path::PathProvider;
use shared::event::TauriEvent;
use shared::payload::ImagePayload;

pub fn receive_image(app_handle: &tauri::AppHandle, images: &Arc<Mutex<PathProvider>>) {
    let main_window = app_handle.get_webview_window("main").unwrap();

    if let Some(img) = images.lock().unwrap().get() {
        main_window
            .emit(TauriEvent::ReceiveImage.as_ref(), ImagePayload { uri: img })
            .unwrap();
    }
}
