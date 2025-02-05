use tungstenite::{connect, Message};
use base64::prelude::*;
use rand::Rng;

fn get_input() -> String {
    let mut line = String::new();
    let poss_err = std::io::stdin().read_line(&mut line);    
    if poss_err.is_err() {
        return String::from("");
    } 
    return line.parse().unwrap();
}

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
}

fn main() {
    
    let (mut socket, response) = connect("ws://localhost:8080/wsAtt").expect("Can't Connect");
    drop(response);
    println!("connected to server");
    
    socket.send(Message::text("attacker connected".to_string())).unwrap();
    
    loop {
        let input = get_input().replace("\n", ""); 
       
        if input != "" {
            let input = encode(input); 
            socket.send(Message::text(input)).unwrap();
        }
    
    }

    // socket.close(None);
}

