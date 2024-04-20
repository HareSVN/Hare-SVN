
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



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
            .args(["/C", "svn checkout ", &name, " ~/Documents/SVN"])
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
        let ouput = std::process::Command::new("sh")
            .args(["-c ", "svn checkout ", &name, " ~/Documents/SVN"])
            .output()
            .expect("Repo failed to chekc out");
        println!("test: {:?}", ouput);
    };    
}

/*
Check status gets a list of files and whether they are modified 
*/
#[tauri::command]
fn status(name: String) -> String { //return type temp for debugging
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("svn")
            .arg("status") //change to svn Status
            .current_dir("/")
            .output()
            .expect("Failed to run the svn status command");
    }
    else {
        let output = std::process::Command::new("svn")
            .arg("status")//change to svn status
            .current_dir("/home/dominic/Documents/SVN") //note hardcoded change dominic to be userprofile 
            .output();
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
        .invoke_handler(tauri::generate_handler![ 
                                                checkout, 
                                                status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
