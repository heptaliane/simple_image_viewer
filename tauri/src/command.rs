use std::path::PathBuf;

use shared::payload::FilePathPayload;
use tauri::{command, State};

use crate::path::PathRepository;

fn create_file_path_payload(paths: Vec<PathBuf>) -> FilePathPayload {
    FilePathPayload {
        paths: paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
    }
}

#[command]
pub fn get_files(state: State<'_, Box<dyn PathRepository>>) -> Result<FilePathPayload, String> {
    let paths = state.files()?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_next_directory(
    state: State<'_, Box<dyn PathRepository>>,
) -> Result<FilePathPayload, String> {
    state.next_directory()?;
    let paths = state.files()?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_prev_directory(
    state: State<'_, Box<dyn PathRepository>>,
) -> Result<FilePathPayload, String> {
    state.prev_directory()?;
    let paths = state.files()?;
    Ok(create_file_path_payload(paths))
}
