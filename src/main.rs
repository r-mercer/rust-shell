#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;

// use crate::actions::write;
use crate::commands::builtin;
use crate::commands::ext;
mod actions;
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

        let writeout = false;
        let stringpath;

        if com_arr.1.contains("> ") {
            let write_arr = com_arr.1.split_once("> ");
            com_arr.1 = write_arr.unwrap_or_default().0;
            stringpath = write_arr.unwrap_or_default().1;
            writeout = true;
        }

        if com_arr.0 == "exit" {
            exit = false;
            continue;
        }
        if Ok(builtin::check_string_output(com_arr.0, com_arr.1)) {
            continue;
        }

        if let Some(_path) = find_executable_in_path(&com_arr.0) {
            Command::new(com_arr.0)
                .args(ext::parse_comm(com_arr.1))
                .status()
                .unwrap_or_default();
        } else {
            line_out("{com_arr.0}: command not found");
            // println!("{}: command not found", com_arr.0);
        }
        if writeout {
            line_out();
        } else {
            actions::write::to_file(stringpath, com_arr.0);
        }
    }
}

fn line_out(line: &str) {
    io::stdout().write_all(line.as_bytes());
    main();
}
