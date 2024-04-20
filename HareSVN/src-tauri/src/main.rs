
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
fn status(name: String) -> std::vec::Vec<String> { //return type temp for debugging
    if cfg!(target_os = "windows"){
        let output = std::process::Command::new("svn")
            .arg("status") //change to svn Status
            .arg("-v")
            .current_dir("/")
            .output();
        //println!("{:?}", output.expect("Error").stdout.into_iter().map(|x: u8| x as char).collect::<String>());
        let ret_string: String = output.expect("Error doing thing").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        println!("{}", ret_string);
        let mut ret_files: std::vec::Vec<String> = Vec::new();
        let mut line: String = String::new();
        for c in ret_string.chars() {
            if c == '\n'{
                ret_files.push(line);
                line = String::new();
            }
            line.push(c);
        }
        return ret_files;
    }
    else {
        let output = std::process::Command::new("svn")
            .arg("status")//change to svn status
            .arg("-v")
            .current_dir("/home/dominic/Documents/SVN/repo") //note hardcoded change dominic to be userprofile 
            .output();
        //println!("{:?}", output.expect("Error").stdout.into_iter().map(|x: u8| x as char).collect::<String>());
        let ret_string: String = output.expect("Error doing thing").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        println!("{}", ret_string);
        let mut ret_files: std::vec::Vec<String> = Vec::new();
        let mut line: String = String::new();
        for c in ret_string.chars() {
            if c == '\n'{
                ret_files.push(line);
                line = String::new();
            }
            line.push(c);
        }
        return ret_files;
    }
}

#[tauri::command]
fn commit(selected_files: std::vec::Vec<String>, message: String) -> () {
    if cfg!(target_os = "windows"){
        let output = std::process::Command::new("svn")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .current_dir("/home/dominic/Documents/SVN")//change dominic to userid fix this to work on windows ds
            .output();
    }
    else{
        let output = std::process::Command::new("svn")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .current_dir("/home/dominic/Documents/SVN") //change dominic to userid
            .output();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ 
                                                checkout, 
                                                status,
                                                commit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
