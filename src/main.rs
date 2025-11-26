#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;

use crate::commands::builtin;
use crate::commands::ext;
mod commands;

fn main() {
    let mut exit = true;

    while exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let mut char: &str = " ";
        let mut com_str = command.trim();

        if com_str.starts_with("'") {
            char = "'";
            com_str = com_str.strip_prefix("'").unwrap_or_default();
        } else if com_str.starts_with("\"") {
            char = "\"";
            com_str = com_str.strip_prefix("\"").unwrap_or_default();
        }
        command = com_str.to_string();

        let com_arr = command
            .split_once(char)
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
                .args(ext::parse_comm(com_arr.1))
                .status()
                .expect(&path.to_string_lossy());
        } else {
            println!("{}: command not found", com_arr.0);
        }
    }
}
