#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod utils;

use crate::commands::login_id_ouc_edu_cn;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login_id_ouc_edu_cn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
