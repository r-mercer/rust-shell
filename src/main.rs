#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io;
use std::io::Write;
use std::process::Command;

// use crate::actions::write;
use crate::commands::builtin::exec_builtin;
use crate::commands::command_type::{get_type, CommandType, LineCommand};
use crate::helpers::parsers::{get_tokens, parse_comm};
mod actions;
mod commands;
mod helpers;

fn main() {
    // io::stdout().flush().unwrap();
    let mut exit = true;

    while exit {
        // io::stdout().flush().unwrap();
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut stringput = String::new();
        io::stdin().read_line(&mut stringput).unwrap();

        let tokens = get_tokens(stringput);
        let command = LineCommand::from_tokens(tokens);

        // let command = parse_input(stringput);

        if command.execute == "exit" {
            exit = false;
            continue;
        }

        let _ = output(command);

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

fn parse_input(mut command: String) -> LineCommand {
    let mut char: &str = " ";
    let mut com_str = command.trim();

    if com_str.starts_with("'") {
        char = "'";
        com_str = com_str.strip_prefix("'").unwrap_or_default();
        command = com_str.to_string();
    } else if com_str.starts_with("\"") {
        char = "\"";
        com_str = com_str.strip_prefix("\"").unwrap_or_default();
        command = com_str.to_string();
    }

    let com_arr = command
        .split_once(char)
        .unwrap_or_else(|| (command.trim(), ""));

    let mut com = LineCommand {
        file_path: None,
        to_file: false,
        type_of: get_type(&command),
        execute: com_arr.0.to_string(),
        args: None,
        arg: Some(com_arr.1.to_string()),
    };

    com.args = Some(parse_comm(com_arr.1));
    com
}

fn output(inc_command: LineCommand) -> Result<(), io::Error> {
    let command = inc_command.to_owned();
    let mut ret = String::new();

    if command.arg.is_some() && command.args.is_none() {
        println!("{}", "should we be printing this?")
    }

    match command.type_of {
        CommandType::BuiltIn => {
            ret = exec_builtin(command)?.expect("something");
        }
        CommandType::OnUserPATH => {
            Command::new(command.execute)
                .args(command.args.unwrap_or_default())
                .status()
                .unwrap_or_default();
        }
        CommandType::Absolute => {
            Command::new(command.execute)
                .args(command.args.unwrap_or_default())
                .status()
                .unwrap_or_default();
        }
    }

    if command.to_file == true {
        println!("We could have printed here {}", command.execute);
        // actions::write::to_file(command.file_path.unwrap_or_default(), ret)?;
    } else {
        line_out(ret);
    }
    Ok(())
}

fn line_out(line: String) {
    println!("{}", line);
    // let _ = io::stdout().write_all(line.as_bytes());
    // main();
}
