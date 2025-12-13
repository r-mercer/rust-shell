#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io;
use std::io::Write;
use std::process::Command;

// use crate::actions::write;
// use crate::commands::builtin::exec_builtin;
use crate::commands::command_type::{LineCommand, OutputType};
use crate::helpers::parsers::get_tokens;
mod actions;
mod commands;
mod helpers;

fn main() {
    let mut exit = false;

    while !exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut stringput = String::new();
        io::stdin().read_line(&mut stringput).unwrap();

        let tokens = get_tokens(stringput);
        let command_vec = LineCommand::vec_from_tokens(tokens);

        for command in command_vec {
            if command.executable == "exit" {
                exit = true;
                continue;
            }

            let result = command.execute_command();
            match result {
                Ok(t) => match t.output_type {
                    OutputType::Str => {
                        let _ = actions::write::output_to(&command, &t);
                    }
                    OutputType::Vec => {
                        let _ = actions::write::output_to_vec(&command, &t);
                    }
                    OutputType::None => {}
                },
                // } else {
                //     println!("Output: {}", t.output_str.unwrap_or_default());
                // }
                Err(e) => println!("{}", e.output_str.unwrap_or_default()),
            }
        }
        // let _ = output(command);

        // if Ok(builtin::check_string_output(com_arr.0, com_arr.1)) {
        //     continue;
        // }

        // if let Some(_path) = find_executable_in_path(&com_arr.0) {
        //     Command::new(com_arr.0)
        //         .args(ext::parse_comm(com_arr.1))
        //         .status()
        //         .unwrap_or_default();
        // } else {
        //     line_out("{com_arr.0}: command not found");
        //     // println!("{}: command not found", com_arr.0);
        // }
    }
}

// fn parse_input(mut command: String) -> LineCommand {
//     let mut char: &str = " ";
//     let mut com_str = command.trim();
//
//     if com_str.starts_with("'") {
//         char = "'";
//         com_str = com_str.strip_prefix("'").unwrap_or_default();
//         command = com_str.to_string();
//     } else if com_str.starts_with("\"") {
//         char = "\"";
//         com_str = com_str.strip_prefix("\"").unwrap_or_default();
//         command = com_str.to_string();
//     }
//
//     let com_arr = command
//         .split_once(char)
//         .unwrap_or_else(|| (command.trim(), ""));
//
//     let mut com = LineCommand {
//         file_path: None,
//         to_file: false,
//         type_of: get_type(&command),
//         executable: com_arr.0.to_string(),
//         args: None,
//     };
//
//     com.args = Some(parse_comm(com_arr.1));
//     com
// }
//
// fn line_out(line: String) {
//     println!("{}", line);
//     // let _ = io::stdout().write_all(line.as_bytes());
//     // main();
// }
