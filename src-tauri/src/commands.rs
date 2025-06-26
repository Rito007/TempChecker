use tauri::{AppHandle, State};
use crate::AppState;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    temperature_limit: i16,
}

fn config_path(app: &AppHandle) -> std::path::PathBuf {
    let mut dir = app.path().app_data_dir().expect("Sem diretório de dados");
    dir.push("config.json");
    dir
}

fn save_temperature_limit(app: &AppHandle, limit: i16) {
    let config = Config { temperature_limit: limit };
    let path = config_path(app);
    let _ = std::fs::create_dir_all(path.parent().unwrap());
    let _ = fs::write(path, serde_json::to_string_pretty(&config).unwrap());
}

#[tauri::command]
pub fn set_limit_command(new_limit: i16, app: AppHandle, state: State<'_, AppState>) {
    {
        let mut limit = state.temperature_limit.lock().unwrap();
        *limit = new_limit;
    }
    save_temperature_limit(&app, new_limit);
    println!("🎯 Novo limite salvo: {new_limit}°C");
}

#[tauri::command]
pub fn get_limit_command(state: State<'_, AppState>) -> i16 {
    *state.temperature_limit.lock().unwrap()
}

#[tauri::command]
pub fn get_temperature_command(state: State<'_, AppState>) -> i16 {
    *state.temperature.lock().unwrap()
}