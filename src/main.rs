use std::io::{self, BufRead, Write};
use std::process::Command;

fn main() {
  println!("Let's do it!");

  vish_loop();
}

fn vish_loop() {
    let mut working = true;

    while(working) {
        print!(">>");
        io::stdout().flush().ok().expect("Ops... Something went wrong. :(");

        if let Some(mut command) = interpret(read_command()) {
            match command.output() {
                Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout)),
                Err(err) => println!("Command error: {}", err)
            }

            working = true;
        } else {
            working = false;
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

fn parse(command_line: String) -> Option<ShellArgs> {
    let parts = command_line.split_whitespace().collect();
    return Some(ShellArgs::from(parts))
}

fn interpret(line: String) -> Option<Command> {
    if let Some(shellargs) = parse(line) {
        let mut command = Command::new(&shellargs.command);
        for arg in &shellargs.args { command.arg(arg); }
        println!("arguments {:?}", shellargs);

        return Some(command);
    } else {

        None
    }
}

#[derive(Debug)]
struct ShellArgs {
    command: String,
    args: Vec<String>
}
impl ShellArgs {
    pub fn from(args: Vec<&str>) -> Self {
        let cmd = args[0].to_string();
        let arguments = args[1..].iter().map(|s| s.to_string()).collect();
        ShellArgs { command: cmd, args: arguments }
    }
}

struct ShellCommand;
