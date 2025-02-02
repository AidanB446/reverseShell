use powershell_script::PsScriptBuilder;

fn main() {
    
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(true) 
        .print_commands(false) 
        .build();

    let output = ps.run("echo hello there").unwrap();
        
    println!("{:?}", output.stdout().unwrap());
}


