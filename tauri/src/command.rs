use std::path::{Path, PathBuf};

use shared::payload::FilePathPayload;
use tauri::command;

use crate::path::{get_child_files, get_directory, next_directory, prev_directory};

fn create_file_path_payload(paths: Vec<PathBuf>) -> FilePathPayload {
    FilePathPayload {
        paths: paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
    }
}

#[command]
pub fn get_files(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_directory(Path::new(&path))?;

    let paths = get_child_files(&directory, &sort)?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_next_directory(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_directory(Path::new(&path))?;
    let next_directory = next_directory(&directory, &sort)?;

    let paths = get_child_files(&next_directory, &sort)?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_prev_directory(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_directory(Path::new(&path))?;
    let next_directory = prev_directory(&directory, &sort)?;

    let paths = get_child_files(&next_directory, &sort)?;
    Ok(create_file_path_payload(paths))
}
