#[macro_use]
extern crate clap;

use clap::App;
use std::process::{Command, Stdio};
use std::time::SystemTime;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let command: Vec<_> = matches.values_of("COMMAND").unwrap().collect();

    let start_time = SystemTime::now();

    match Command::new(command[0])
        .args(&command[1..])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(_) => {
            eprintln!("This command does not exist.")
        }
    }

    match start_time.elapsed() {
        Ok(time) => println!("{}", time.as_millis()),
        Err(e) => eprintln!("{:?}", e)
    }
}
