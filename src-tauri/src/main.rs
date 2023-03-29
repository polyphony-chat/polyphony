// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod auth;
pub mod backend;
pub mod instance;
pub mod user;

use crate::auth::auth as authcrate;
use crate::backend::backend::Backend;
use crate::backend::backend::FosscordBackend;
use crate::instance::instance::Instance;
use crate::instance::instance::InstanceType;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let instance: Instance<FosscordBackend> = Instance::new(
        String::from("Local Fosscord Instance"),
        String::from("http://localhost:3001"),
        InstanceType::Fosscord,
        FosscordBackend::new(String::from("http://localhost:3001")),
    );

    let register: authcrate::RegisterParams = authcrate::RegisterParams {
        email: "test4@mailprovider.com".to_string(),
        password: "Unimportant123##1".to_string(),
        username: "Test4".to_string(),
        consent: true,
        fingerprint: "whatdoiputhere1972346789127890343".to_string(),
        date_of_birth: "2000-01-01".to_string(),
        promotional_email_opt_in: false,
        invite: None,
        gift_code_sku_id: None,
        captcha_key: None,
    };

    let login: authcrate::LoginParams = authcrate::LoginParams {
        login: register.email.clone(),
        password: register.password.clone(),
        undelete: false,
        captcha_key: None,
        login_source: None,
        gift_code_sku_id: None,
    };

    let reg_resp = authcrate::register_fosscord(&instance.conn, register).await;

    let login_resp = authcrate::login_fosscord(&instance.conn, login).await;

    println!(
        "Instance online: {}",
        instance.conn.check_health().await.to_string()
    );

    println!("Registration: {}", reg_resp);

    println!("Login: {}", login_resp);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
