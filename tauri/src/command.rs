use std::fs;
use std::path::Path;

use shared::payload::FilePathPayload;
use tauri::command;

#[command]
pub fn get_files(path: String) -> Result<FilePathPayload, String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err(format!("Path `{:?}` does not exist", path));
    }

    let directory = if path.is_dir() {
        path
    } else {
        match path.parent() {
            Some(parent) => parent,
            _ => return Err(format!("Cannot get parent directory of `{:?}`", path)),
        }
    };

    match fs::read_dir(&directory) {
        Ok(entries) => {
            let paths: Vec<String> = entries
                .filter(|p| p.is_ok())
                .map(|p| p.unwrap().path())
                .filter(|p| p.is_file())
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            return Ok(FilePathPayload { paths: paths });
        }
        Err(e) => Err(format!("Error reading directory: `{:?}`", e)),
    }
}
