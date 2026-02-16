// Tauri Imports
use tauri::async_runtime::Mutex;
use tauri::{Manager, State};

use crate::core::node::Node;
use crate::managers::graph_manager::GraphManager;

#[tauri::command]
pub async fn add_graph(state: State<'_, Mutex<GraphManager>>) -> Result<(), ()> {
    let mut graph_manager = state.lock().await;

    graph_manager.add_graph();

    println!("Added graph to GraphManager");

    Ok(())
}

#[tauri::command]
pub async fn get_node(graph_id: usize, node_id: usize, state: State<'_, Mutex<GraphManager>>) -> Result<&Node, ()> {
    let mut graph_manager = state.lock().await;

    let graph = graph_manager.get_graph(id);

    Ok(graph.get_node(node_id))
}