use std::path::{Path, PathBuf};

use shared::payload::FilePathPayload;
use tauri::command;

use crate::path::{get_child_files, next_directory, prev_directory};

fn get_target_directory(path: String) -> Result<PathBuf, String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err(format!("Path `{:?}` does not exist", path));
    }

    match path.is_dir() {
        true => Ok(path.to_path_buf()),
        false => match path.parent() {
            Some(parent) => Ok(parent.to_path_buf()),
            _ => Err(format!("Cannot get parent directory of `{:?}`", path)),
        },
    }
}

fn create_file_path_payload(paths: Vec<PathBuf>) -> FilePathPayload {
    FilePathPayload {
        paths: paths.iter().map(|p| p.to_string_lossy().into_owned()).collect()
    }
}

#[command]
pub fn get_files(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_target_directory(path)?;

    let paths = get_child_files(&directory, &sort)?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_next_directory(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_target_directory(path)?;
    let next_directory = next_directory(&directory, &sort)?;

    let paths = get_child_files(&next_directory, &sort)?;
    Ok(create_file_path_payload(paths))
}

#[command]
pub fn get_prev_directory(path: String) -> Result<FilePathPayload, String> {
    let sort = |p: &PathBuf| p.clone();
    let directory = get_target_directory(path)?;
    let next_directory = prev_directory(&directory, &sort)?;

    let paths = get_child_files(&next_directory, &sort)?;
    Ok(create_file_path_payload(paths))
}
