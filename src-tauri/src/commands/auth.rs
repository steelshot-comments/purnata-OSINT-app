use serde::{Deserialize, Serialize};
use crate::AppState;
use reqwest::header::AUTHORIZATION;

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub mfa_required: bool,
    pub ticket: Option<String>,
    pub access_token: Option<String>,
}

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    email: String,
    password: String,
) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/auth/v1/token?grant_type=password", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(data["error_description"].as_str().unwrap_or("Login failed").to_string());
    }

    let factors = data["user"]["factors"].as_array();
    let mfa_required = factors.map_or(false, |f| !f.is_empty());

    Ok(AuthResponse {
        mfa_required,
        ticket: if mfa_required { data["access_token"].as_str().map(String::from) } else { None },
        access_token: if !mfa_required { data["access_token"].as_str().map(String::from) } else { None },
    })
}

#[tauri::command]
pub async fn verify_totp(
    state: tauri::State<'_, AppState>,
    code: String,
    ticket: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/auth/v1/verify", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .header(AUTHORIZATION, format!("Bearer {}", ticket))
        .json(&serde_json::json!({ "type": "totp", "token": code }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    if data.get("error").is_some() {
        return Err(data["error_description"].as_str().unwrap_or("MFA failed").to_string());
    }
    Ok(data)
}

#[tauri::command]
pub async fn signup(
    state: tauri::State<'_, AppState>,
    email: String,
    password: String,
    display_name: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/auth/v1/signup", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .header(AUTHORIZATION, format!("Bearer {}", state.supabase.anon_key))
        .json(&serde_json::json!({
            "email": email,
            "password": password,
            "data": { "display_name": display_name }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(body["msg"].as_str().unwrap_or("Signup failed").to_string());
    }
    Ok(body)
}

#[tauri::command]
pub async fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/logout/", state.mobile_auth_ip))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn enroll_mfa(state: tauri::State<'_, AppState>, session_token: String) -> Result<serde_json::Value, String> {
    let res = reqwest::Client::new()
        .post(format!("{}/auth/v1/factors", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .header("Authorization", format!("Bearer {}", session_token))
        .json(&serde_json::json!({ "friendly_name": "My Desktop App", "factor_type": "totp" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn verify_factor(
    state: tauri::State<'_, AppState>,
    factor_id: String,
    code: String,
    session_token: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    // Verify the factor to "activate" it during setup
    let res = client
        .post(format!(
            "{}/auth/v1/factors/{}/verify",
            state.supabase.url, factor_id
        ))
        .header("apikey", &state.supabase.anon_key)
        .header(AUTHORIZATION, format!("Bearer {}", session_token))
        .json(&serde_json::json!({ "code": code }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(data)
}