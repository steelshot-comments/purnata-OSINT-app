use crate::Serialize;
use uuid::{uuid, Uuid};

use crate::AppState;
use std::collections::HashMap;

#[derive(Serialize)]
struct BasePayload {
    user_id: Uuid,
    graph_id: Uuid,
    project_id: Uuid,
}

#[derive(Serialize)]
struct NodeCreateRequest {
    #[serde(flatten)]
    base: BasePayload,
    nodes: Vec<InternalNodeData>,
}

#[derive(Serialize)]
struct InternalNodeData {
    labels: Vec<String>, // Note: FastAPI uses "labels" (plural)
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
    let payload = BasePayload {
        user_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
        graph_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
        project_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
    };

    let response = reqwest::Client::new()
        .get(format!("{}/graph", state.mobile_neo4j_api))
        .json(&payload)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read body: {}", e))
}

#[tauri::command]
pub async fn add_node_to_graph(
    state: tauri::State<'_, AppState>,
    label: String, // Incoming from Svelte
    properties: HashMap<String, String>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let base = BasePayload {
        user_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
        graph_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
        project_id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
    };

    // Construct the request matching the FastAPI NodeCreateRequest model
    let payload = NodeCreateRequest {
        base,
        nodes: vec![InternalNodeData {
            labels: vec![label], // Put the single label into a list
            properties,
        }],
    };

    let response = client
        .post(format!("{}/add-node", state.mobile_neo4j_api))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Node successfully created in Neo4j".into())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".into());
        Err(format!("Failed to create node: {}", error_text))
    }
}

#[tauri::command]
pub async fn delete_node_from_graph(
    state: tauri::State<'_, AppState>,
    id_value: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    // 1. Build the full payload matching NodeDeleteRequest
    let payload = serde_json::json!({
        "user_id": "550e8400-e29b-41d4-a716-446655440000",
        "graph_id": "550e8400-e29b-41d4-a716-446655440000",
        "project_id": "550e8400-e29b-41d4-a716-446655440000",
        "id": id_value
    });

    
    let url = format!("{}/delete-node", state.mobile_neo4j_api.trim_end_matches('/'));

    let response = client
        .delete(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok(format!("Node {} successfully deleted", id_value))
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".into());
        Err(format!("Delete failed ({}): {}", status, error_text))
    }
}

#[tauri::command]
pub async fn clone_node(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    reqwest::Client::new()
        .put(format!("{}/clone-node", state.mobile_neo4j_api))
        .json(&serde_json::json!({ "id": id }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
