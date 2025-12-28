// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use tauri::{generate_handler, Manager};
use tauri_plugin_cli::CliExt;

mod command;
mod path;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(generate_handler![
            command::get_files,
            command::get_next_directory,
            command::get_prev_directory,
        ])
        .setup(|app| {
            let args = app.cli().matches()?.args;
            let filename = args["filename"].value.to_string();
            let sort = |p: &PathBuf| p.clone();
            app.manage(path::FilePathRepository::new(Path::new(&filename), sort));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
