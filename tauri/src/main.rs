// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};

use shared::event::TauriEvent;
use tauri::Listener;
use tauri_plugin_cli::CliExt;
use tauri::Manager;

mod event;
mod path;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let args = app.cli().matches().unwrap().args;

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
                app.listen(TauriEvent::RequestImage.as_ref(), move |_| {
                    event::receive_image(&app_handle, &images);
                });
            }

            {
                let images = images.clone();
                let app_handle = app.app_handle();
                app.listen(TauriEvent::MoveNext.as_ref(), move |_| {
                    images.lock().unwrap().move_next();
                    event::receive_image(&app_handle, &images);
                });
            }

            {
                let images = images.clone();
                let app_handle = app.app_handle();
                app.listen(TauriEvent::MovePrev.as_ref(), move |_| {
                    images.lock().unwrap().move_prev();
                    event::receive_image(&app_handle, &images);
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
