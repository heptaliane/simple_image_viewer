// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod payload;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.emit_to(
                "main",
                "image_uri",
                payload::ImageURIPayload {
                    uri: "public/tauri.svg".into(),
                },
            ).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
