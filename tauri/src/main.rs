// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use shared::event::TauriEvent;
use tauri::{Listener, Manager};
use tauri_plugin_cli::CliExt;

mod command;
mod path;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            // Setup Repository
            let args = app.cli().matches()?.args;
            let filename = args["filename"]
                .value
                .as_str()
                .expect("String arg is expected for filename");
            let sort = |p: &PathBuf| p.clone();
            let directory = path::FileDirectoryRepository::new(Path::new(&filename), sort);
            let repo = path::FilePathRepository::new(directory, sort);
            let boxed: Box<dyn path::PathRepository> = Box::new(repo);
            app.manage(boxed);

            // Setup listener
            {
                let handle = app.handle().clone();
                app.listen(TauriEvent::RequestFile.as_ref(), move |_| {
                    command::request_file(&handle);
                });
            }
            {
                let handle = app.handle().clone();
                app.listen(TauriEvent::NextFile.as_ref(), move |_| {
                    command::next_file(&handle);
                });
            }
            {
                let handle = app.handle().clone();
                app.listen(TauriEvent::PrevFile.as_ref(), move |_| {
                    command::prev_file(&handle);
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
