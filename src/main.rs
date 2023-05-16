use std::env;
use std::fs;
use std::process;

enum Command {
    Show,
    Add(String, String),
    Rm(u32),
    All,
    Help,
}

impl Command {
    fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() == 1 {
            return Ok(Command::Show);
        }else if args.len() == 2{
            if args[1] == "help"{
                return Ok(Command::Help);
            }else if args[1] == "all" {
                return Ok(Command::All);
            }else{
                return Err("Inaccurate command. For information on available commands, use:\n\ttasks help");
            }
        }else{
            todo!();
        }
    }
}

fn command_help() {
    println!("There are several commands that you can use:");
    println!("\ttasks                       - show all tasks");
    println!("\ttasks add \"Watch a film\"    - add task \"Buy a milk\"");
    println!("\ttasks add \"Buy a milk\" high - add a priority task (low, mid:default, high)");
    println!("\ttasks rm 2                  - delete task with number 2");
    println!("\ttasks all                   - show completed and incomplete tasks");
}

fn execute_command(command: &Command) {
    match command {
        Command::Help => command_help(),
        _ => (),
    }
}
fn main() {    
    let args: Vec<String> = env::args().collect();
    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    execute_command(&command);
}
