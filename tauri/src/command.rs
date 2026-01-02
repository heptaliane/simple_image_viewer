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

pub fn request_file(app: &AppHandle) {
    match app.state::<Box<dyn PathRepository>>().file() {
        Ok(path) => match send_file(app, path) {
            Err(e) => println!(
                "[ERROR] Event '{:?}' failed: {:?}",
                TauriEvent::RequestFile.as_ref(),
                e
            ),
            _ => (),
        },
        Err(e) => println!(
            "[ERROR] Event '{:?}' failed: {:?}",
            TauriEvent::RequestFile.as_ref(),
            e
        ),
    }
}
pub fn next_file(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    match repo.next() {
        Err(e) => println!(
            "[ERROR] Event '{:?}' failed: {:?}",
            TauriEvent::NextFile.as_ref(),
            e
        ),
        _ => match repo.file() {
            Ok(path) => match send_file(app, path) {
                Err(e) => println!(
                    "[ERROR] Event '{:?}' failed: {:?}",
                    TauriEvent::NextFile.as_ref(),
                    e
                ),
                _ => (),
            },
            Err(e) => println!(
                "[ERROR] Event '{:?}' failed: {:?}",
                TauriEvent::NextFile.as_ref(),
                e
            ),
        },
    }
}
pub fn prev_file(app: &AppHandle) {
    let repo = app.state::<Box<dyn PathRepository>>();
    match repo.prev() {
        Err(e) => println!(
            "[ERROR] Event '{:?}' failed: {:?}",
            TauriEvent::PrevFile.as_ref(),
            e
        ),
        _ => match repo.file() {
            Ok(path) => match send_file(app, path) {
                Err(e) => println!(
                    "[ERROR] Event '{:?}' failed: {:?}",
                    TauriEvent::PrevFile.as_ref(),
                    e
                ),
                _ => (),
            },
            Err(e) => println!(
                "[ERROR] Event '{:?}' failed: {:?}",
                TauriEvent::PrevFile.as_ref(),
                e
            ),
        },
    }
}
