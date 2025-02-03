
use std::process::{Command, Stdio};

fn main() {

    let initial_command = r#"-a /usr/bin"#.split(" ").collect::<Vec<&str>>();

    let output = Command::new("ls")
        .stdout(Stdio::piped())
        .args(initial_command)
        .spawn()
        .expect("Failed to execute command");
        

    let status = output.wait_with_output().unwrap();

    println!("Output {:?}", String::from_utf8(status.stdout).unwrap());
}

