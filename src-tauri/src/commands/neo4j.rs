use crate::Serialize;

use crate::AppState;
use std::collections::HashMap;

#[derive(Serialize)]
struct NodePayload {
    label: String,
    properties: HashMap<String, String>,
}

#[derive(Serialize)]
struct DeletePayload {
    label: String,
    id_property: String,
    id_value: String,
}

#[tauri::command]
pub async fn fetch_graph(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let response = reqwest::Client::new()
        .get(format!("{}/graph", state.mobile_neo4j_api))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    response.text().await.map_err(|e| format!("Failed to read body: {}", e))
}

#[tauri::command]
pub async fn add_node_to_graph(
    state: tauri::State<'_, AppState>, 
    label: String, 
    properties: HashMap<String, String>
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    // We wrap the data in a payload struct for clean serialization
    let payload = NodePayload { label, properties };

    let response = client
        .post(format!("{}/graph/node", state.mobile_neo4j_api))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Node successfully created in Neo4j".into())
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".into());
        Err(format!("Failed to create node: {}", error_text))
    }
}

#[tauri::command]
pub async fn delete_node_from_graph(
    state: tauri::State<'_, AppState>,
    label: String,
    id_property: String,
    id_value: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let cloned_id = id_value.clone();

    let payload = DeletePayload {
        label,
        id_property,
        id_value: cloned_id,
    };

    let response = client
        .delete(format!("{}/graph/node", state.mobile_neo4j_api))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok(format!("Node {} successfully deleted", id_value))
    } else {
        Err(format!("Delete failed with status: {}", response.status()))
    }
}