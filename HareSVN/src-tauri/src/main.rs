// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String{
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "mkdir ~/test"])
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
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
fn checkout(name: String) -> () {
    if cfg!(target_os = "windows") {
        if !std::path::Path::new("~/Documents/SVN").exists(){
            let _ = std::process::Command::new("cmd")
                .args(["/C", "mkdir ~/test"])
                .output()
                .expect("failed to execute process");
        }
        let _ = std::process::Command::new("cmd")
            .args(["/C", "cd ~/Documents/SVN"])
            .output()
            .expect("Changing dir to SVN");
        let _ = std::process::Command::new("cmd")
            .args(["/C", "svn checkout", &name])
            .output()
            .expect("Repo failed to chekc out");
    } 
    else {
        if !std::path::Path::new("~/Documents/SVN").exists(){
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg("mkdir ~/Documents/SVN")
                .output()
                .expect("failed to execute process");
        }
        let _ = std::process::Command::new("sh")
            .args(["-c", "cd ~/Documents/SVN"])
            .output()
            .expect("Changing dir to SVN");
        let _ = std::process::Command::new("sh")
            .args(["-c", "svn checkout", &name])
            .output()
            .expect("Repo failed to chekc out");
    };
}

/*
Check status gets a list of files and whether they are modified 
*/
#[tauri::command]
fn check_status(name: String) -> String { //return type temp for debugging
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("sh")
            .args(["/C", "cd ~/Documents/SVN"])
            .output()
            .expect("Failed to change directories");
        let _ = std::process::Command::new("cmd")
            .args(["/C", "touch this.txt"]) //change to svn Status
            .output()
            .expect("Failed to run the svn status command");
    }
    else {
        /*
        let _ = std::process::Command::new("sh")
            .args(["-c", "cd ~/Documents/SVN"])
            .output()
            .expect("Failed to change directories");
        */
        let path: &std::path::Path = std::path::Path::new("~/Documents/SVN");
        let _ = std::env::set_current_dir(&path);
        let _ = std::process::Command::new("sh")
            .args(["-c", "touch ", "sttuff.txt"]) //change to svn status
            .output()
            .expect("Failed to run the svn status command");
    }
    "dhc".to_string()
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
        .invoke_handler(tauri::generate_handler![greet, 
                                                checkout, 
                                                check_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
