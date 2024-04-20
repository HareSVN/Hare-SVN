// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[tauri::command]
fn checkout(repo_name: String) {
    if !std::path::Path::new("Documents/SVN/").exists() {
        let _ = std::process::Command::new("cd");
        let _ = std::process::Command::new("cd Documents");
        let _ = std::process::Command::new("mkdir SVN");
    }
    let _ = std::process::Command::new("cd");
    let _ = std::process::Command::new("cd Documents/SVN");
    let _ = std::process::Command::new("svn checkout")
        .arg(repo_name);
        
}

#[tauri::command]
fn check_status(folder_name: String) {
    let check_command = std::process::Command::new("svn status")
        .output();
}

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
