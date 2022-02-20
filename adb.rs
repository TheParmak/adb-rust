use std::process::Command;
use std::string::String;

pub fn adb(args1: &str, adb_path: &str) {
    let splid = args1.split_whitespace();
    let vec: Vec<&str> = splid.collect();
    let adb = adb_path;
    let output = Command::new(adb).args(vec).output().expect("failed to execute process");
    
    if !&output.stdout.is_empty() { 
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else if !&output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

}
