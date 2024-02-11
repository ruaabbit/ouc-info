#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod utils;

use crate::commands::{get_pdf_blob, get_score_pdf_url, login_id_ouc_edu_cn};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login_id_ouc_edu_cn,
            get_score_pdf_url,
            get_pdf_blob
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
