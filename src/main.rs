#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

mod commands;

use commands::cmd::{handle_echo, handle_type};

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
            let cmd_len = split_command.len();
            if cmd == "echo" {
                handle_echo(&(split_command[1..cmd_len].join(" ")));
            } else if cmd == "type" {
                handle_type(&(split_command[1..cmd_len].join(" ")));
            } else {
                println!("{}: command not found", command)
            }
        }
    }
}
