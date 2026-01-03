use std::path::PathBuf;

use shared::event::TauriEvent;
use shared::payload::FilePayload;
use tauri::{AppHandle, Emitter, Manager};

use crate::path::PathRepository;

fn send_file(app: &AppHandle, path: PathBuf) -> Result<(), String> {
    match app.emit(
        TauriEvent::ReceiveFile.as_ref(),
        FilePayload {
            path: path.to_string_lossy().into_owned(),
        },
    ) {
        Err(e) => Err(e.to_string()),
        _ => Ok(()),
    }
}
fn handle_update_path_result(
    app: &AppHandle,
    repo: &Box<dyn PathRepository>,
    event: TauriEvent,
    result: Result<(), String>,
) {
    match result {
        Err(e) => println!("[ERROR] Event '{:?}' failed: {:?}", event.as_ref(), e),
        _ => match repo.file() {
            Ok(path) => match send_file(app, path) {
                Err(e) => println!("[ERROR] Event '{:?}' failed: {:?}", event.as_ref(), e),
                _ => (),
            },
            Err(e) => println!("[ERROR] Event '{:?}' failed: {:?}", event.as_ref(), e),
        },
    }
}

pub fn request_file(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    let result = Ok(());
    handle_update_path_result(app, &repo, TauriEvent::RequestFile, result);
}
pub fn next_file(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    let result = repo.next();
    handle_update_path_result(app, &repo, TauriEvent::NextFile, result);
}
pub fn prev_file(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    let result = repo.prev();
    handle_update_path_result(app, &repo, TauriEvent::PrevFile, result);
}
pub fn next_directory(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    let result = repo.next_directory();
    handle_update_path_result(app, &repo, TauriEvent::NextDirectory, result);
}
pub fn prev_directory(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    let result = repo.prev_directory();
    handle_update_path_result(app, &repo, TauriEvent::PrevDirectory, result);
}
