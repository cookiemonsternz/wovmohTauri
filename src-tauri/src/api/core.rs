// Tauri Imports
use tauri::async_runtime::Mutex;
use tauri::{Manager, State};

use crate::managers::graph_manager::GraphManager;

#[tauri::command]
pub async fn add_graph(state: State<'_, Mutex<GraphManager>>) -> Result<(), ()> {
    let mut graph_manager = state.lock().await;

    graph_manager.add_graph();

    println!("Added graph to GraphManager");

    Ok(())
}