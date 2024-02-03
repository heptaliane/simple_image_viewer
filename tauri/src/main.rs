// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};

use shared::event::TauriEvent;
use tauri::Manager;

mod event;
mod path;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let args = app.get_cli_matches().unwrap().args;

            let filenames: Vec<String> = args["filenames"]
                .value
                .as_array()
                .unwrap()
                .iter()
                .map(|filename| filename.as_str().unwrap().to_string())
                .collect();
            let images = Arc::new(Mutex::new(path::PathProvider::new(&filenames)));

            {
                let images = images.clone();
                let app_handle = app.app_handle();
                let _ = app.listen_global(TauriEvent::RequestImage.as_ref(), move |_| {
                    event::receive_image(&app_handle, &images);
                });
            }

            {
                let images = images.clone();
                let app_handle = app.app_handle();
                let _ = app.listen_global(TauriEvent::MoveNext.as_ref(), move |_| {
                    images.lock().unwrap().move_next();
                    event::receive_image(&app_handle, &images);
                });
            }

            {
                let images = images.clone();
                let app_handle = app.app_handle();
                let _ = app.listen_global(TauriEvent::MovePrev.as_ref(), move |_| {
                    images.lock().unwrap().move_prev();
                    event::receive_image(&app_handle, &images);
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
