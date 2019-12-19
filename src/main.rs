#[macro_use]
extern crate clap;

use clap::{App, AppSettings};
use std::process::{Command, Stdio, exit};
use std::time::SystemTime;

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml)
        .setting(AppSettings::TrailingVarArg)
        .get_matches();

    let command: Vec<_> = matches.values_of("COMMAND").unwrap().collect();
    let disable_output = matches.is_present("disable-output");

    let num_times_res = matches
        .value_of("times")
        .unwrap_or("1")
        .parse::<i32>();
    let num_times = match num_times_res {
        Ok(num) => num,
        Err(_) => {
            eprintln!("That is not an integer");
            exit(1);
        }
    };

    let mut times_vec: Vec<u128> = vec![];

    for _ in 0..num_times {
        let start_time = SystemTime::now();
        run_command(command.clone(), disable_output);
        match start_time.elapsed() {
            Ok(time) => {
                times_vec.push(time.as_millis())
            }
            Err(e) => eprintln!("{:?}", e)
        }
    }

    for elapsed in times_vec {
        println!("{}", elapsed);
    }
}


fn run_command(command: Vec<&str>, disable_output: bool) {
    match Command::new(command[0])
        .args(&command[1..])
        .stdin(if disable_output {
            Stdio::null()
        } else { Stdio::inherit() })
        .stderr(if disable_output {
            Stdio::null()
        } else { Stdio::inherit() })
        .stdout(if disable_output {
            Stdio::null()
        } else { Stdio::inherit() })
        .spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(_) => {
            eprintln!("This command does not exist.")
        }
    }
}
