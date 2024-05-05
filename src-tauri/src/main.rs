// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lcd_core::{
    config, init,
    miner::entry::{MachineInfo, MachineRecord, PoolConfig},
    query_machine_records_by_time, reboot, scan, watching, MinersLibConfig,
};
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::fs;
use tokio::runtime::Runtime;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(50) // 设置工作线程的数量
        .enable_all()
        .build()
        .unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn scan_machines(
    ip: String,
    offset: i32,
    count: i32,
    timeout: i64,
) -> Result<Vec<MachineInfo>, String> {
    scan(RUNTIME.handle().clone(), &ip, offset, count, timeout).await
}

#[tauri::command]
async fn reboot_machines(ips: Vec<String>) -> Result<(), String> {
    reboot(RUNTIME.handle().clone(), ips).await
}

#[tauri::command]
async fn config_machines(ips: Vec<String>, account: Vec<PoolConfig>) -> Result<i64, String> {
    config(RUNTIME.handle().clone(), ips, account).await
}

#[tauri::command]
async fn watch_machines(ips: Vec<String>, timeout: i64) -> Result<Vec<MachineInfo>, String> {
    watching(RUNTIME.handle().clone(), ips, timeout).await
}

#[tauri::command]
fn query_ip_records(ip: String, start: i64, end: i64) -> Result<Vec<MachineRecord>, String> {
    query_machine_records_by_time(ip, start, end)
}

fn init_log(app_path: &str) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[Console] {d} - {l} -{t} - {m}{n}",
        )))
        .build();

    // Create a file appender with dynamic log path
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[File] {d} - {l} - {t} - {m}{n}",
        )))
        .build(app_path.to_owned() + "/log/info.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // Use this config
    log4rs::init_config(config).unwrap();
}

fn main() {
    //init();
    tauri::Builder::default()
        .setup(|app| {
            let app_data_path = app
                .path_resolver()
                .app_data_dir()
                .expect("error read data dir");
            println!("app data path: {:?}", app_data_path);
            if !app_data_path.exists() {
                fs::create_dir_all(&app_data_path).expect("failed to create directory");
            }
            init_log(app_data_path.to_str().unwrap());
            init(&MinersLibConfig {
                is_need_db: true,
                app_path: app_data_path.to_str().unwrap().to_owned(),
                feishu_app_id: "".to_owned(),
                feishu_app_secret: "".to_owned(),
                feishu_bot: "".to_owned(),
                db_keep_days: 20,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_machines,
            reboot_machines,
            config_machines,
            watch_machines,
            query_ip_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
