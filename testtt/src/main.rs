use std::process::{Command, Stdio};


fn run(cmmd : String) -> String {
    
    // run the passed command and get output
    let mut cmmd = cmmd.split(" ").collect::<Vec<&str>>();
    
    println!("{:?}", cmmd);

    // replace command with cmd to run in windows
    let output = Command::new(cmmd.remove(0))
        .stdout(Stdio::piped())
        .args(cmmd)
        .spawn();
    
    if output.is_err() {
        return String::from("Command failed to execute on output");
    }
    
    let output = output.expect("command failed to execute");

    let status = output.wait_with_output();

    if status.is_err() {
        return String::from("Command Failed to execute on status"); 
    }
   
    return String::from_utf8(status.unwrap().stdout).unwrap();
}


fn main() {
    
    let value = run(String::from("ls -a"));
    
    println!("{}", value);


}

