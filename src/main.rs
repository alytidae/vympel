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
        }else if args.len() == 2 && args[1] == "all" {
            return Ok(Command::All);
        }else if args.len() == 2 && args[1] == "help" {
            return Ok(Command::Help);
        }else if args.len() >= 2{
            todo!();
        }else {
            Err("Inaccurate command. For information on available commands, use:\n tasks help") 
        }
    }
}

fn show_help() {
    println!("There are several commands that you can use:\n");
    println!("tasks                          - show all tasks");
    println!("tasks add \"Buy a milk\"         - add task \"Buy a milk\"");
    println!("tasks add \"Watch a film\" high  - add a priority task (low, mid:default, high)");
    println!("tasks rm 2                     - delete task with number 2");
    println!("tasks all                      - show completed and incomplete tasks");
}

fn execute_command(command: &Command) {
    match command {
        Command::Help => show_help(),
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
