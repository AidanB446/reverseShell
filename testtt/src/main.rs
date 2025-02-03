
use std::process::{Command, Stdio};

fn main() {

    let initial_command = r#"echo "hello" > file.txt "#.split(" ").collect::<Vec<&str>>();

    let output = Command::new("cmd")
        .args(initial_command)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");



    println!("{}", String::from_utf8_lossy(&output.stdout));
}


