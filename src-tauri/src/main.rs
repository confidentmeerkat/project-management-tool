// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn main() {
    let app = app::App::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app::get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
