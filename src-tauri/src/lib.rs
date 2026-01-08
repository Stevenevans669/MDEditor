use std::sync::Mutex;

pub mod commands;

pub struct AppState {
    pub file_path: Mutex<Option<String>>,
}
