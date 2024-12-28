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
        let path_env = std::env::var("PATH").unwrap();
        let split = &mut path_env.split(":");
        if let Some(path) =
            split.find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
        {
            println!("{} is {}/{}", command, path, command);
        } else {
            println!("{} not found", command);
        }
    }
}
