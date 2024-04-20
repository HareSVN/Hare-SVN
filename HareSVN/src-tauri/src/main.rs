// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
/*
#[tauri::command]
fn checkout(folder_name: String) {
    
}

#[tauri::command]
fn check_status() {
    //maybe
    let check_command = std::process::Command::new("svn checkout")
        .output()
        .expect("Failed to execute command");
}
*/
#[tauri::command]
fn commit(selected_files: std::vec::Vec<String>) {
    let mut file_string: String = String::new();
    for file in selected_files {
        file_string = " ".to_string() + &file;
    }
    let commit_command = std::process::Command::new("svn")
        .arg(&file_string)
        .output();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
