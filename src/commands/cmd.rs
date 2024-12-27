use std::io::{self, Write};

pub fn handle_echo(command: &str) {
    println!("{}", command);
    io::stdout().flush().unwrap();
}

pub fn handle_type(command: &str) {
    const VALID_CMD: &[&str] = &["echo", "type", "exit"];
    if VALID_CMD.contains(&command) {
        println!("{} is a shell builtin", command);
        io::stdout().flush().unwrap();
    } else {
        println!("{}: not found", command);
        io::stdout().flush().unwrap();
    }
}
