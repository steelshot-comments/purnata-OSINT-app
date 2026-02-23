use crate::AppState;

#[tauri::command]
pub async fn run_transform(
    state: tauri::State<'_, AppState>, 
    source: String, 
    node_id: String, 
    query: String
) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/run/{}", state.osint_api_url, source))
        .json(&serde_json::json!({ "query": query, "node_id": node_id }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_action_map(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let url = format!("{}/action-map", state.osint_api_url);
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.text().await.map_err(|e| e.to_string())
}