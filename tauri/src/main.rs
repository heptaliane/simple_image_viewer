// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use shared::event::TauriEvent;
use shared::payload::ImagePayload;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let _ = app.listen_global(TauriEvent::RequestImage.as_ref(), move |_| {
                let main_window = app_handle.get_window("main").unwrap();
                main_window
                    .emit(
                        TauriEvent::ReceiveImage.as_ref(),
                        ImagePayload {
                            uri: "public/tauri.svg".to_string(),
                        },
                    )
                    .unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
