use std::process::Command;
use std::string::String;

pub fn adb_static_path(args1: &str) {
    let adb_path = "bin/adb"; //set your adb path here
    let splid = args1.split_whitespace();
    let vec: Vec<&str> = splid.collect();
    let output = Command::new(adb_path).args(vec).output().expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

