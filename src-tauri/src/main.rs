// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn identity(a: f64, _b: Option<f64>) -> f64 {
    a
}

#[tauri::command]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[tauri::command]
fn sub(a: f64, b: f64) -> f64 {
    a - b
}

#[tauri::command]
fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[tauri::command]
fn div(a: f64, b: f64) -> f64 {
    a / b
}

#[tauri::command]
fn power(a: f64, b: f64) -> f64 {
    a.powf(b)
}

#[tauri::command]
fn sin(a: f64) -> f64 {
    a.sin()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            identity,
            add,
            sub,
            mul,
            div,
            power,
            sin,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
