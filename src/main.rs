// TODO: Find a way how to delete this dublicate
use clap::Parser;
use std::process;


mod args;
mod commands;
mod config;

use args::{Command, AppArgs};
use config::Config;

impl Command {
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

    fn execute(&self, config: &Config) -> Result<(), &'static str>{
        match self {
            Command::Show => self.show(config),
            Command::Add { .. } => self.add(config),
            Command::Rm { .. } => self.rm(config),
            Command::All => self.all(config),
        }
    }
}



fn main() {    
    let config = Config::build().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    
    let args = AppArgs::parse();
    let command = args.command.unwrap_or(Command::Show);
    dbg!(command);

    //match command.execute(&config) {
    //    Ok(_) => (),
    //    Err(error) => println!("{error}"),
    //};

}
