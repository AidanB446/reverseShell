use tungstenite::{connect, Message};
use base64::prelude::*;
use rand::Rng;

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
    return String::from_utf8(BASE64_STANDARD.decode(inp).unwrap()).unwrap();
}

fn main() {
    
    let (mut socket, response) = connect("ws://localhost:8080/wsAtt").expect("Can't Connect");
    drop(response);
    println!("connected to server");
    
    let cli_message : String = String::from("this is from attacker");


    socket.send(Message::text(cli_message)).unwrap();
    socket.send(Message::text("this is from attacker".to_string())).unwrap();
    socket.send(Message::text("this is from attacker too".to_string())).unwrap();
    
    loop {
        let msg = socket.read().expect("Error reading message");        
        println!("Recieved: {msg}");
    }
    
    // socket.close(None);
}


