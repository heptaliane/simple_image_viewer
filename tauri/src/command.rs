use std::path::PathBuf;

use shared::payload::FilePayload;
use tauri::{command, State};

use crate::path::PathRepository;

fn create_file_payload(path: PathBuf) -> FilePayload {
    FilePayload {
        path: path.to_string_lossy().into_owned(),
    }
}

#[command]
pub async fn get_file(state: State<'_, Box<dyn PathRepository>>) -> Result<FilePayload, String> {
    let path = state.file()?;
    Ok(create_file_payload(path))
}

#[command]
pub async fn get_next_file(
    state: State<'_, Box<dyn PathRepository>>,
) -> Result<FilePayload, String> {
    state.next()?;
    let path = state.file()?;
    Ok(create_file_payload(path))
}

#[command]
pub async fn get_prev_file(
    state: State<'_, Box<dyn PathRepository>>,
) -> Result<FilePayload, String> {
    state.prev()?;
    let path = state.file()?;
    Ok(create_file_payload(path))
}
