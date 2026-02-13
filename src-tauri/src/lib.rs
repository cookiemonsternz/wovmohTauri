// Tauri Imports
use tauri::async_runtime::Mutex;
use tauri::{Manager, State};

use crate::managers::graph_manager::GraphManager;

mod api;
mod core;
mod managers;
mod nodes;
mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, api::core::add_graph])
        .setup(|app| {
            app.manage(Mutex::new(GraphManager::new()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
