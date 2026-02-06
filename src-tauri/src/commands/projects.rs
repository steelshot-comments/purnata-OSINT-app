use crate::AppState;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
}

#[tauri::command]
pub async fn get_projects(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let res = reqwest::Client::new()
        .get(format!("{}/projects/", state.mobile_auth_ip))
        .json(&serde_json::json!({"userID": 4})) // Note: Hardcoded ID 4 from original
        .send()
        .await
        .map_err(|e| e.to_string())?;

    res.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_project(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    reqwest::Client::new()
        .delete(format!("{}/projects/delete", state.mobile_auth_ip))
        .json(&serde_json::json!({"projectID": id}))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}