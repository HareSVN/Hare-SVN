use std::process::Output;


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn get_user()->String{
    let mut user = std::process::Command::new("whoami").output().expect("bad get").stdout.into_iter().map(|x| x as char).collect::<String>();
    user.pop();
    return user
}

/*
Redesign so it is not stupid
May not have to have a return value
*/
#[tauri::command]
fn checkout(url:String, name: String) -> () {
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
        if !std::path::Path::new(&format!("~/Documents/SVN/{name}")).exists(){
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("mkdir ~/Documents/SVN/{name}"))
                .output()
                .expect("failed to execute process");
        }
        let user = get_user();
        let ouput = std::process::Command::new("svn")
            .arg("checkout")
            .arg(&url)
            .arg(format!("/home/{user}/Documents/SVN/{name}"))
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
    let user = get_user();
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
        let mut real_ret: std::vec::Vec<String> = Vec::new();
        for file in ret_files{
            if file.starts_with("/"){
                real_ret.push(file);
            }
        }
        println!("files: {:?}", real_ret);
        return real_ret;
        //return ret_files;
    }
    else {
        let output = std::process::Command::new("svn")
            .arg("status")//change to svn status
            .arg("-v")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //note hardcoded change dominic to be userprofile 
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
        let mut real_ret: std::vec::Vec<String> = Vec::new();
        for file in ret_files{
            println!("file: {:?}", file);
            real_ret.push(file.split_ascii_whitespace().last().unwrap().to_string())
        }
        println!("files: {:?}", real_ret);
        return real_ret;
        //return ret_files;
    }
}

/*
Untested
Runs svn commit in repo of choice
*/
#[tauri::command]
fn commit(message: String, name:String) -> () {
    let user: String = get_user();
    println!("in the function: {}| {}", message, name);
    if cfg!(target_os = "windows"){
        let output = std::process::Command::new("svn")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))//change dominic to userid fix this to work on windows ds
            .output();
    }
    else{
        let output = std::process::Command::new("svn")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //change dominic to userid
            .output();
        println!("{:?}", output);
    }

}

/*
Untested
The filelist will be a list of file to be added to svn 
name will be the folder they are in
How do we actually signify this -> todo
*/
#[tauri::command]
fn add(filelist: std::vec::Vec<String>, name: String) -> () {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("svn")
            .arg("add")
            .args(filelist)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //NOT WINDOWS!!!!!!!!!!!! <--------------------------
            .output();
    }
    else{
        println!("path: {:?}", format!("/home/{user}/Documents/SVN/{name}"));
        let output = std::process::Command::new("svn")
            .arg("add")
            .args(filelist)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))//repo is temporary
            .output();
        println!("{:?}", output);

    }
}

/*
Untested
Function to execute svn update, must select repository first
*/
#[tauri::command]
fn update(name: String) -> () {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("svn")
            .arg("update")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //NOT WINDOWS !!!!!!!!!!
            .output();        
    }
    else{
        let _ = std::process::Command::new("svn")
            .arg("update")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))
            .output();
    }
}

/*
Takes the svn info command and reduces it to the revision number
*/
#[tauri::command]
fn revision(name: String) -> String {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let output = std::process::Command::new("svn")
            .arg("info")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //NOT WINDOWS
            .output();
        let info: String = output.expect("Error converting to String in revision function").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        let mut revision_line: String = String::new();
        for l in info.lines() {
            if l.contains("Revision"){
                revision_line = l.to_string();
            }
        }
        let mut ret: String = String::new();
        for c in revision_line.chars() {
            if c.is_digit(10) {
                ret.push(c);
            }
        }
        return ret;
    }
    else{
        let output = std::process::Command::new("svn")
            .arg("info")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))
            .output();
        let info: String = output.expect("Error converting to String in revision function").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        let mut revision_line: String = String::new();
        for l in info.lines() {
            if l.contains("Revision"){
                revision_line = l.to_string();
            }
        }
        let mut ret: String = String::new();
        for c in revision_line.chars() {
            if c.is_digit(10) {
                ret.push(c);
            }
        }
        return ret;
    }
}
/*
Untested
This returns the entire result from the svn log command, may format as needed
*/
#[tauri::command]
fn history(name: String) -> String {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let output = std::process::Command::new("svn")
            .arg("log")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))//NOT WINDOWS!!
            .output();
        let ret: String = output.expect("Error converting to String in revision function").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        return ret;
    }
    else{
        let output = std::process::Command::new("svn")
            .arg("log")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))
            .output();
        let ret: String = output.expect("Error converting to String in revision function").stdout.into_iter().map(|x: u8| x as char).collect::<String>();
        return ret;
    }
}

#[tauri::command]
fn revert(name: String) -> () {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("svn")
            .arg("revert")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))
            .output();
    }
    else{
        let _ = std::process::Command::new("svn")
            .arg("revert")
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))
            .output();
    }
}

#[tauri::command]
fn delete(filelist: std::vec::Vec<String>, name: String) -> () {
    let user: String = get_user();
    if cfg!(target_os = "windows"){
        let _ = std::process::Command::new("svn")
            .arg("delete")
            //.arg("--force")
            .args(filelist)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}")) //NOT WINDOWS!!!!!!!!!!!! <--------------------------
            .output();
    }
    else{
        println!("path: {:?}", format!("/home/{user}/Documents/SVN/{name}"));
        let output = std::process::Command::new("svn")
            .arg("delete")
            .args(filelist)
            .current_dir(format!("/home/{user}/Documents/SVN/{name}"))//repo is temporary
            .output();
        println!("{:?}", output);

    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ 
                                                checkout, 
                                                status,
                                                commit,
                                                add,
                                                update,
                                                revision,
                                                history,
                                                revert,
                                                delete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}