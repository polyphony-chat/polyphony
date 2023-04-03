// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod auth;
pub mod backend;
pub mod instance;
pub mod serenity;
pub mod user;

use auth::auth::RegisterParams;
use backend::{Backend, URLBundle};

use crate::auth::auth::register_spacebar;
use crate::backend::SpacebarBackend;
use crate::instance::instance::Instance;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/* #[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} */

#[tokio::main]
async fn main() {
    let urls = URLBundle::new(
        String::from("http://localhost:3001/api/"),
        String::from("wss://localhost:3001"),
        String::from("http://localhost:3001"),
    );
    let token = register_spacebar(
        &urls,
        RegisterParams {
            email: String::from("testmail@mailman.com"),
            password: String::from("thisisatest"),
            username: String::from("Testenby1312"),
            consent: true,
            fingerprint: String::from("what"),
            invite: None,
            date_of_birth: String::from("2000-01-01"),
            gift_code_sku_id: None,
            captcha_key: None,
            promotional_email_opt_in: false,
        },
    )
    .await;
    let instance = Instance::new(
        String::from("Test"),
        urls.clone(),
        SpacebarBackend::new(token, urls.clone()).await,
    );
    println!("URL for API: {}", instance.urls.get_api());
    println!("Healthy? {}", SpacebarBackend::check_health(&urls).await);

    /* tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application"); */
}
