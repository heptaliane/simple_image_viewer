// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use shared::event::TauriEvent;
use tauri::{generate_handler, Emitter};
use tauri_plugin_cli::CliExt;

mod command;
mod path;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![command::get_files])
        .invoke_handler(generate_handler![command::get_next_directory])
        .invoke_handler(generate_handler![command::get_prev_directory])
        .setup(|app| {
            let args = app.cli().matches()?.args;
            let filename = args["filename"].value.to_string();
            app.emit(
                TauriEvent::Initialize.as_ref(),
                command::get_files(filename),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
