


fn get_input() -> String {
    let mut line = String::new();
    let poss_err = std::io::stdin().read_line(&mut line); 

    if poss_err.is_err() {
        return "Failed to get Input".to_string();
    } 

    return line.parse().unwrap();
}

fn main() {
    println!("{}", get_input());
}

