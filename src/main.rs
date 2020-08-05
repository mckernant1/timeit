#[macro_use]
extern crate clap;

use clap::{App, AppSettings};
use std::process::{Command, Stdio, exit, Child};
use std::time::SystemTime;

fn main() {
    let yml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yml);
    app = app.setting(AppSettings::TrailingVarArg);
    let matches = app.get_matches();

    let cmd: Vec<&str> = matches.values_of("COMMAND").unwrap().collect::<Vec<&str>>();
    let disable_output = matches.is_present("disable-output");
    let parallel = matches.is_present("parallel");
    let continue_on_failure = matches.is_present("continue-on-failure");

    let num_times_res = matches
        .value_of("number-of-times-to-run")
        .unwrap_or("1")
        .parse::<i32>();
    let num_times = match num_times_res {
        Ok(num) => num,
        Err(_) => {
            eprintln!("That is not an integer");
            exit(1);
        }
    };

    let mut children: Vec<(Child, SystemTime)> = vec![];
    let mut times_vec: Vec<u128> = vec![];
    let command = cmd.to_owned();

    for _ in 0..num_times {
        if parallel {
            let t = run_command(command.to_owned(), true);
            children.push((t, SystemTime::now()));
        } else {
            let start_time = SystemTime::now();
            let exit_status = run_command(command.to_owned(), disable_output.to_owned())
                .wait().unwrap();

            if !exit_status.success() && !continue_on_failure {
                break;
            }

            match start_time.elapsed() {
                Ok(time) => {
                    times_vec.push(time.as_millis())
                }
                Err(e) => eprintln!("{:?}", e)
            }
        }
    }

    for (mut child, time) in children {
        child.wait().unwrap();
        match time.elapsed() {
            Ok(time) => {
                println!("{}", time.as_millis())
            }
            Err(e) => eprintln!("{:?}", e)
        }
    }

    for elapsed in times_vec {
        println!("{}", elapsed);
    }
}

fn run_command(command: Vec<&str>, disable_output: bool) -> Child {
    return match Command::new(command[0])
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
        Ok(child) => child,
        Err(_) => {
            eprintln!("This command does not exist.");
            exit(1);
        }
    }
}
