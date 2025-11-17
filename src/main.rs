#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;

use crate::commands::builtin;
mod commands;

fn main() {
    let mut exit = true;

    while exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let com_arr = command
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| (command.trim(), ""));

        if com_arr.0 == "exit" {
            exit = false;
            continue;
        }
        if builtin::check(com_arr.0, com_arr.1) {
            continue;
        }
        if let Some(path) = find_executable_in_path(&com_arr.0) {
            Command::new(com_arr.0)
                .args(com_arr.1.split(' '))
                .status()
                .expect(&path.to_string_lossy());
        } else {
            println!("{}: command not found", com_arr.0);
        }
    }
}
