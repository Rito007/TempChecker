use tauri::{AppHandle, State};
use crate::AppState;
use std::{fs, env};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Serialize, Deserialize)]
struct Config {
    temperature_limit: f32,
}






#[tauri::command]
pub fn set_limit_command(new_limit: f32, app: AppHandle, state: State<'_, AppState>) {
    {
        let mut limit = state.temperature_limit.lock().unwrap();
        *limit = new_limit;
    }
    save_temperature_limit(new_limit);
    println!("🎯 Novo limite salvo: {new_limit}°C");
}

#[tauri::command]
pub fn get_limit_command(state: State<'_, AppState>) -> f32 {
    *state.temperature_limit.lock().unwrap()
}

#[tauri::command]
pub fn get_temperature_command(state: State<'_, AppState>) -> f32 {
    *state.temperature.lock().unwrap()
}