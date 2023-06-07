use std::env;
use std::fs;
use std::process;
use directories::ProjectDirs;
use serde::Deserialize;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct AppArgs {
    /// The command to execute
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show all tasks
    Show,
    /// Add task 
    Add{
        task_body: String, 
        #[clap(subcommand)]
        task_priority: TaskPriority,
    },
    /// Remove task
    Rm {
        /// Number of task to remove
        task_number: u32,
    },
    /// Show completed and incomplete tasks
    All,
    // TODO: Edit(u32),
}


#[derive(Debug, Subcommand)]
enum TaskPriority {
    Low,
    Mid,
    High,
}

#[derive(Deserialize, Debug)]
struct Config {
}

impl Command {
    //fn build(args: &[String]) -> Result<Command, &'static str> {
        //if args.len() == 1 {
        //    return Ok(Command::Show);
        //}else if args.len() == 2 {
        //    if args[1] == "help"{
        //        return Ok(Command::Help);
        //    }else if args[1] == "all" {
        //        return Ok(Command::All);
        //    }else{
        //        return Err("Inaccurate command. For information on available commands, use:\n\ttasks help");
        //    }
        //}else if args.len() == 3 && args[1] == "rm"{
        //    match args[2].parse::<u32>(){
        //        Ok(n) => return Ok(Command::Rm(n)),
        //        Err(_) => return Err("Number of task must be integer. Like this:\n\ttasks rm 2     - remove task number 2"),
        //    }
        //}else if args.len() == 3 && args[1] == "add" {
        //    Ok(Command::Add(args[2].clone(), TaskPriority::Mid))
        //}else if args.len() == 4 && args[1] == "add" {
        //    if args[3] == "low" {
        //        Ok(Command::Add(args[2].clone(), TaskPriority::Low))
        //    }else if args[3] == "mid" {
        //        Ok(Command::Add(args[2].clone(), TaskPriority::Mid))
        //    }else if args[3] == "high" {
        //        Ok(Command::Add(args[2].clone(), TaskPriority::High))
        //    }else{
        //        Err("The priority of the task should be one of 3 options:\n\tlow\n\tmid\n\thigh")
        //    }
        //}else{
        //    return Err("Inaccurate command. For information on available commands, use:\n\ttasks help");
        //}
    //}

    //fn help(&self) -> Result<(), &'static str> {
    //    println!("There are several commands that you can use:");
    //    println!("\tvympel                       - show all tasks");
    //    println!("\tvympel add \"Watch a film\"    - add task \"Buy a milk\"");
    //    println!("\tvympel add \"Buy a milk\" high - add a priority task (low, mid:default, high)");
    //    println!("\tvympel rm 2                  - remove task number 2");
    //    println!("\tvympel all                   - show completed and incomplete tasks");
    //    Ok(())
    //}
    
    fn show(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn add(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn rm(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn all(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }

    //fn execute(&self, config: &Config) -> Result<(), &'static str>{
    //    match self {
    //        Command::Help => self.help(),
    //        Command::Show => self.show(config),
    //        Command::Add(_,_) => self.add(config),
    //        Command::Rm(_) => self.rm(config),
    //        Command::All => self.all(config),
    //    }
    //}
}

impl Config {
    fn build() -> Result<Config, String> {
        if let Some(proj_dirs) = ProjectDirs::from(
            "dev",
            "vympel",
            "vympel",
        ) {
            let config_dir = proj_dirs.config_dir();
            
            let config_file = fs::read_to_string(
                config_dir.join("config.toml")
            );

            let config: Config = match config_file {
                Ok(file) => {
                    match toml::from_str(&file) {
                        Ok(toml_deserialize) => toml_deserialize,
                        Err(err) => return Err(format!("Error in config: {}", err.message().to_string())),
                    }
                },
                Err(_) => Config {
                }
            };
            
            return Ok(config);
        }
        Err(String::from("No valid home directory path"))
    }
}

fn main() {    
    let config = Config::build().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    
    let args = AppArgs::parse();
    dbg!(args);
    //let args: Vec<String> = env::args().collect();
    //let command = Command::build(&args).unwrap_or_else(|err| {
    //    println!("{err}");
    //    process::exit(1);
    //});
    
    //match command.execute(&config) {
    //    Ok(_) => (),
    //    Err(error) => println!("{error}"),
    //};

}
