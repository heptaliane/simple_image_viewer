use std::path::{Path, PathBuf};

use shared::payload::FilePathPayload;
use tauri::command;

use crate::path::get_child_files;

#[command]
pub fn get_files(path: String) -> Result<FilePathPayload, String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err(format!("Path `{:?}` does not exist", path));
    }

    let directory = match path.is_dir() {
        true => path,
        false => match path.parent() {
            Some(parent) => parent,
            _ => return Err(format!("Cannot get parent directory of `{:?}`", path)),
        },
    };

    let sort = |p: &PathBuf| path;
    let paths = get_child_files(&directory, &sort);
    match paths {
        Ok(entries) => Ok(FilePathPayload {
            paths: entries
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
        }),
        Err(e) => Err(format!("Error reading directory: `{:?}`", e)),
    }
}
