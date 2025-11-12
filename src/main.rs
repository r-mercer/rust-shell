use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
mod builtin;

fn main() {
    let mut exit = true;

    while exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        // let com_arr = command.trim().split_once(' ').unwrap_();
        let com_arr = command.trim().split_once(' ').unwrap_or_else(|| (command.trim(), ""));

        if com_arr.0 == "exit" {
            // I think we can just break here??
            exit = false;
            continue;
        }
        if builtin::check(com_arr.0, com_arr.1) {
            continue;
        }
        if let Some(path) = find_executable_in_path(&com_arr.0) {
            Command::new(path)
                .args(com_arr.1.split(' '))
                .spawn();
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}

// fn parse_comd() {}
