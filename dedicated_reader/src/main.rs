use tungstenite::{connect, Message};
use base64::prelude::*;

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

    socket.send(Message::text("reader connected")).unwrap();
    
    loop {
        let msg = &socket.read().expect("Error reading message");        
        let message_from_client = decode((msg.to_text().unwrap()).to_string());
        println!("{}", message_from_client); 
    }

}
