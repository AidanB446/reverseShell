
use std::process::{Command};

fn main() {

    let initial_command = r#"echo "hello" >>> file.txt "#.split(" ").collect::<Vec<&str>>();

    let mut output = Command::new("cmd")
        .args(initial_command)
        .spawn()
        .expect("Failed to execute command");

    let status = output.wait().unwrap();

    println!("{:?}", status);

}


