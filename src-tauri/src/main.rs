// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![
    cfg_attr(not(debug_assertions), 
    windows_subsystem = "windows"
)]

mod commands;

use commands::check_git_status;
use tauri::generate_handler;


fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![check_git_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
