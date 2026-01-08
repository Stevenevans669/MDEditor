#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::Manager;

use mdeditor::commands;
use mdeditor::AppState;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).cloned();

    if file_path.is_none() {
        eprintln!("Usage: mdeditor <file_path>");
        std::process::exit(2);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .manage(AppState {
            file_path: Mutex::new(file_path),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_file_path,
            commands::read_file,
            commands::save_file,
            commands::exit_app,
        ])
        .setup(|app| {
            let state = app.state::<AppState>();
            if let Some(path) = state.file_path.lock().unwrap().as_ref() {
                if let Some(window) = app.get_webview_window("main") {
                    let filename = std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("MDEditor");
                    let _ = window.set_title(filename);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
