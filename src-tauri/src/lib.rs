use tauri::{AppHandle, Manager, State, WindowEvent};
use std::sync::{Arc, Mutex};
use::std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub mod sound;
pub mod temptray;
pub mod taskbar;
pub mod commands;
pub mod tempmonitor;


use crate::temptray::build_tray;
use crate::tempmonitor::start_temperature_monitoring;
use crate::commands::*;

#[derive(Clone)]
pub struct AppState {
    pub temperature: Arc<Mutex<i16>>,
    pub temperature_limit: Arc<Mutex<i16>>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    temperature_limit: i16,
}


fn config_path() -> std::path::PathBuf {
      let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("config.json");
    path
}

fn load_temperature_limit() -> i16 {
    let path = config_path();
    if let Ok(contents) = std::fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<Config>(&contents) {
            return config.temperature_limit;
        }
    }
    40
}

fn save_temperature_limit(limit: i16) {
    let config = Config { temperature_limit: limit };
    let path = config_path();
    let _ = fs::create_dir_all(path.parent().unwrap());
    let _ = fs::write(path, serde_json::to_string_pretty(&config).unwrap());
}


pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_fs::init())
        .on_window_event(|window, event| {
            if let WindowEvent::Focused(false) = event {
                let _ = window.hide();
            }
        })
        .setup(|app| {
            let temp_limit = load_temperature_limit();
            let state = AppState {
                temperature: Arc::new(Mutex::new(0)),
                temperature_limit: Arc::new(Mutex::new(temp_limit)),
            };
            app.manage(state.clone());
            build_tray(app.handle().clone())?;
            start_temperature_monitoring(app.handle().clone(), state.clone());
            println!("✅ Monitor de temperatura iniciado.");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_limit_command,
            get_limit_command,
            get_temperature_command
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao correr aplicação Tauri");
}