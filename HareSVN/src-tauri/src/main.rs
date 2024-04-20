use std::process::Command;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String{
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "mkdir ~/test"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("mkdir ~/Documents/SVN")
            .output()
            .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    return hello.into_iter().map(|x| x as char).collect::<String>();
}

/*
Redesign so it is not stupid
May not have to have a return value
*/
#[tauri::command]
fn checkout(name: String) -> String {
    if cfg!(target_os = "windows") {
        if !std::path::Path::new("~/Documents/SVN").exists(){
            let output = Command::new("cmd")
                .args(["/C", "mkdir ~/test"])
                .output()
                .expect("failed to execute process");
        }
        let output = Command::new("cmd")
            .args(["/C", "cd ~/Documents/SVN"])
            .output()
            .expect("Changing dir to SVN");
        let output = Command::new("cmd")
            .args(["/C", "svn checkout", &name])
            .output()
            .expect("Repo failed to chekc out");
    } 
    else {
        if !std::path::Path::new("~/Documents/SVN").exists(){
            let output = Command::new("sh")
                .arg("-c")
                .arg("mkdir ~/Documents/SVN")
                .output()
                .expect("failed to execute process");
        }
        let output = Command::new("sh")
            .args(["-c", "cd ~/Documents/SVN"])
            .output()
            .expect("Changing dir to SVN");
        let output = Command::new("sh")
            .args(["-c", "svn checkout", &name])
            .output()
            .expect("Repo failed to chekc out");
    };
    let output = Command::new("sh")
        .args(["echo", "Shut up"])
        .output()
        .expect("Shut up nerd I hate rust");
    
    let hello = output.stdout;
    return hello.into_iter().map(|x| x as char).collect::<String>();
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
        .invoke_handler(tauri::generate_handler![greet, checkout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
