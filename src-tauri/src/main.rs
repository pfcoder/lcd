// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use miners::{detect_miner, init, miner::MinerOperation};
use std::fs;
use tauri::api::path::app_data_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: String) -> String {
    // if ok return miner name, if error return error message
    let result = match detect_miner(&name).await {
        Ok(miner) => miner.info().name,
        Err(e) => e.to_string(),
    };

    format!("Hello, {}! {}!", name, result)
}

fn main() {
    //init();
    tauri::Builder::default()
        .setup(|app| {
            let app_data_path = app
                .path_resolver()
                .app_data_dir()
                .expect("error read data dir");
            //     .path_resolver()
            //     .resolve_resource("log4rs.yaml")
            //     .expect("failed to resolve resource");
            println!("app data path: {:?}", app_data_path);
            if !app_data_path.exists() {
                fs::create_dir_all(&app_data_path).expect("failed to create directory");
            }
            init(app_data_path.to_str().unwrap());
            //let app_dir = app_data_dir().map_err(|e| e.to_string())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
