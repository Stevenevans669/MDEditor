use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::PoisonError;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn get_file_path(state: State<AppState>) -> Result<String, String> {
    state
        .file_path
        .lock()
        .map_err(|e: PoisonError<_>| e.to_string())?
        .clone()
        .ok_or_else(|| "No file path provided".to_string())
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn save_file(path: String, content: String) -> Result<(), String> {
    let path = Path::new(&path);
    let tmp_path = path.with_extension("tmp");

    // Write to temp file
    let mut file = fs::File::create(&tmp_path)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;

    // Flush and sync to disk
    file.sync_all()
        .map_err(|e| format!("Failed to sync file: {}", e))?;

    // Atomic rename
    fs::rename(&tmp_path, path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn exit_app(code: i32) {
    std::process::exit(code);
}
