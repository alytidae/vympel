// TODO: Find a way how to delete this dublicate
use clap::Parser;
use std::process;

mod args;
mod commands;
mod config;
mod tasks;

use args::{AppArgs, Command};
use config::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let args = AppArgs::parse();
    let command = args.command.unwrap_or(Command::Show);
    dbg!(&command);
    dbg!(&config);

    match command.execute(&config) {
        Ok(_) => (),
        Err(error) => println!("{error}"),
    };
}
