// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            let _ = app.listen_global("fetch_image", move |_| {
                let main_window = app_handle.get_window("main").unwrap();
                main_window
                    .emit("image_uri", "public/tauri.svg".to_string())
                    .unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
