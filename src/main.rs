use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

fn main() {
  println!("Let's do it!");

  vish_loop();
}

fn vish_loop() {
    loop {
        print!(">>");
        io::stdout().flush().ok().expect("Ops... Something went wrong. :(");

        if let Some(mut command) = interpret(read_command()) {
            if let Ok(mut child) = command.spawn() {
                match child.wait() {
                    Ok(res) => println!("{}", res),
                    Err(err) => println!("{}", err)
                }
            } else {
                println!("Command not found.");
            }
        }
    }
}

fn read_command() -> String {
    let mut command_line = String::new();
    let stdin = io::stdin();
    stdin.lock()
        .read_line(&mut command_line)
        .expect("I need a command :/");

    command_line
}

fn parse(command_line: String) -> Option<(String, Vec<String>)> {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    let mut iter = parts.iter();

    if let Some(part) = iter.next() {
        let cmd = part.to_string();
        if cmd.is_empty() {
            None
        } else {
            Some((cmd, iter.map(|s| s.to_string()).collect::<Vec<String>>()))
        }
    } else {
        None
    }
}

fn interpret(line: String) -> Option<Command> {
    if let Some((cmd, args)) = parse(line) {
        let mut command = Command::new(cmd);
        for arg in args { command.arg(arg); }

        return Some(command);
    } else {
        None
    }
}
