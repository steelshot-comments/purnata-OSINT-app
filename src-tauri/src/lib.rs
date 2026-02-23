use dotenvy::dotenv;
use serde::{Serialize};
use std::env;
use tauri::{Listener, Builder};
use tauri_plugin_deep_link::DeepLinkExt;
// use uuid;
mod commands;
use commands::{auth, neo4j, projects, osint};

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use tauri_plugin_global_shortcut;


struct SupabaseConfig {
    url: String,
    anon_key: String,
}

// This is the state we will manage
struct AppState {
    supabase: SupabaseConfig,
    mobile_auth_ip: String,
    mobile_neo4j_api: String,
    osint_api_url: String,
}

// #[derive(Serialize)]
// struct RequestBody {
//     user_id: Uuid,
//     project_id: Uuid,
//     graph_id: Uuid,
// }

fn init_env() {
    dotenv().ok();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_env();
    let mobile_auth_ip = env::var("AUTH_API_URL").unwrap();
    let mobile_neo4j_api = env::var("NEO4J_API_URL").unwrap();
    let osint_api_url = env::var("OSINT_API_URL").unwrap();
    let supabase_url = env::var("PUBLIC_SUPABASE_URL").unwrap();
    let supabase_key = env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap();

    let mut builder = Builder::default();

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        builder = builder.plugin(
            tauri_plugin_global_shortcut::Builder::new().build()
        );
    }


    builder
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .manage(AppState {
            supabase: SupabaseConfig {
                url: supabase_url.to_string(),
                anon_key: supabase_key.to_string(),
            },
            mobile_auth_ip,
            mobile_neo4j_api,
            osint_api_url
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
            auth::signup,
            auth::login,
            auth::logout,
            auth::verify_totp,
            auth::verify_factor,
            auth::enroll_mfa,
            projects::get_projects,
            projects::delete_project,
            neo4j::fetch_graph,
            neo4j::add_node_to_graph,
            neo4j::delete_node_from_graph,
            neo4j::clone_node,
            osint::run_transform,
            osint::get_action_map
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
