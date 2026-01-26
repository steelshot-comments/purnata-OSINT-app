use dotenvy::dotenv;
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tauri::Listener;
use tauri_plugin_deep_link::DeepLinkExt;
use uuid::{self, Uuid};

struct SupabaseConfig {
    url: String,
    anon_key: String,
}

// This is the state we will manage
struct AppState {
    supabase: SupabaseConfig,
    auth_api_url: String,
    neo4j_api_url: String,
}

#[derive(Serialize)]
struct RequestBody {
    user_id: Uuid,
    project_id: Uuid,
    graph_id: Uuid,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")] // Converts between Rust snake_case and JS camelCase
struct Project {
    id: i32,
    user_id: i32,
    // This will appear as "userId" in TypeScript automatically via Tauri
}

fn init_env() {
    dotenv().ok();
}

#[tauri::command]
async fn fetch_projects(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let client = reqwest::Client::new();

    // 2. Build the GET request manually
    let response = client
        .get(format!("{}/graph", state.neo4j_api_url))
        .header("Content-Type", "application/json")
        // .body(json_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read body: {}", e))?;

    Ok(body)
}

#[tauri::command]
async fn get_projects(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/projects/", state.auth_api_url))
        .json(&serde_json::json!({"userID": 4}))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    res.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    let client = reqwest::Client::new();
    client
        .delete(format!("{}/projects/delete", state.auth_api_url))
        .json(&serde_json::json!({"projectID": id}))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub mfa_required: bool,
    pub ticket: Option<String>,       // The temporary session/access_token
    pub access_token: Option<String>, // The final token if no MFA
}

#[tauri::command]
async fn login(
    state: tauri::State<'_, AppState>,
    email: String,
    password: String,
) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();

    // Call Supabase Auth Token endpoint
    let res = client
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            state.supabase.url
        ))
        .header("apikey", &state.supabase.anon_key)
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(data["error_description"]
            .as_str()
            .unwrap_or("Login failed")
            .to_string());
    }

    // Check if MFA is required by looking at the "aal" (Authenticator Assurance Level)
    // aal1 = password only, aal2 = MFA completed.
    // If factors exist but we are at aal1, MFA is required.
    let user = &data["user"];
    let factors = user["factors"].as_array();

    let mfa_required = factors.map_or(false, |f| !f.is_empty());

    if mfa_required {
        Ok(AuthResponse {
            mfa_required: true,
            ticket: data["access_token"].as_str().map(String::from), // Use access_token as the ticket
            access_token: None,
        })
    } else {
        Ok(AuthResponse {
            mfa_required: false,
            ticket: None,
            access_token: data["access_token"].as_str().map(String::from),
        })
    }
}

#[tauri::command]
async fn verify_totp(
    state: tauri::State<'_, AppState>,
    code: String,
    ticket: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    // 1. Create a challenge for the factor
    // Note: In a production app, you'd fetch the factor_id from the user's factors list
    // For simplicity, we assume the user has one TOTP factor.

    let auth_header = format!("Bearer {}", ticket);

    // First, we need to challenge the factor (Supabase requirement)
    // You'll need to fetch the factor_id first if not known, but for a standard
    // "verify-after-login" flow, you typically challenge the factor associated with the user.

    // This is the direct verification endpoint for a challenge:
    let res = client
        .post(format!("{}/auth/v1/verify", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .header(AUTHORIZATION, auth_header)
        .json(&serde_json::json!({
            "type": "totp",
            "token": code
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    if data.get("error").is_some() {
        return Err(data["error_description"]
            .as_str()
            .unwrap_or("MFA failed")
            .to_string());
    }

    Ok(data)
}

#[tauri::command]
async fn verify_factor(
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

#[tauri::command]
async fn signup(
    state: tauri::State<'_, AppState>,
    email: String, // Supabase usually uses email for signup
    password: String,
    display_name: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    // 1. Supabase endpoint for signups is /auth/v1/signup
    let url = format!("{}/auth/v1/signup", state.supabase.url);

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        // 2. Supabase requires the anon key in the 'apikey' header
        .header("apikey", &state.supabase.anon_key)
        // 3. And also as a Bearer token in 'Authorization'
        .header(AUTHORIZATION, format!("Bearer {}", state.supabase.anon_key))
        .json(&serde_json::json!({
            "email": email,
            "password": password,
            // 4. Custom data goes into user_metadata
            "data": {
                "display_name": display_name
            }
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
async fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let client = reqwest::Client::new();
    client
        .post(format!("{}/logout/", state.auth_api_url))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn add_node_to_graph(label: String, properties: HashMap<String, String>) -> Result<String, String> {
    println!("Adding {} with properties: {:?}", label, properties);

    // Logic to insert into Neo4j or your graph store goes here

    Ok(format!("Successfully added {} node", label))
}

#[tauri::command]
async fn enroll_mfa(
    state: tauri::State<'_, AppState>,
    session_token: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    // 1. Start enrollment
    let res = client
        .post(format!("{}/auth/v1/factors", state.supabase.url))
        .header("apikey", &state.supabase.anon_key)
        .header("Authorization", format!("Bearer {}", session_token)) // Needs the USER'S token
        .json(&serde_json::json!({
            "friendly_name": "My Desktop App",
            "factor_type": "totp"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    // This 'data' contains 'totp.qr_code' (SVG string) and 'id' (Factor ID)
    Ok(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_env();
    let auth_api_url = env::var("AUTH_API_URL").unwrap();
    let neo4j_api_url = env::var("NEO4J_API_URL").unwrap();
    let supabase_url = env::var("PUBLIC_SUPABASE_URL").unwrap();
    let supabase_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .manage(AppState {
            supabase: SupabaseConfig {
                url: supabase_url.to_string(),
                anon_key: supabase_key.to_string(),
            },
            auth_api_url,
            neo4j_api_url,
        })
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // This handles links sent while the app is ALREADY running
            app.listen_any("deep-link://fallback", |event| {
                println!("Deep link received: {:?}", event.payload());
            });

            // For Windows/Linux: check if app was launched via deep link
            #[cfg(any(windows, target_os = "linux"))]
            app.deep_link()
                .register("com.yeshaya.purnata-svelte-osint")?;
            {
                app.deep_link().on_open_url(|urls| {
                    println!(
                        "Opened with URLs: {:?}",
                        urls.urls()
                            .iter()
                            .map(|url| url.to_string())
                            .collect::<Vec<_>>()
                    );
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            fetch_projects,
            get_projects,
            delete_project,
            login,
            signup,
            logout,
            add_node_to_graph,
            verify_totp,
            verify_factor,
            enroll_mfa
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
