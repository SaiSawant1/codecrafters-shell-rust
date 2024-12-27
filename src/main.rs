#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        shell_loop();
    }
}

fn shell_loop() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let command = input.trim();
    handle_command(command)
}

fn handle_command(command: &str) {
    match command {
        "exit 0" => exit(0),
        _ => {
            let split_command = command.split(" ").collect::<Vec<&str>>();
            let cmd = split_command[0];
            if cmd == "echo" {
                println!("{}", split_command[1..split_command.len()].join(" "));
                io::stdout().flush().unwrap();
            } else {
                println!("{}: command not found", command)
            }
        }
    }
}
