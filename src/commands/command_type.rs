use std::path::PathBuf;

use crate::commands::builtin::BUILTINS;

pub enum CommandType {
    BuiltIn,
    // BuiltInSilent,
    OnUserPATH,
    Absolute,
}

pub struct LineCommand {
    pub to_file: bool,
    pub file_path: Option<String>,
    pub type_of: CommandType,
    pub execute: String,
    pub args: Option<Vec<String>>,
    pub tokens: Vec<String>,
    pub arg: Option<String>,
}

impl LineCommand {
    fn check_output(self) {
        if self.args.is_some() {
            let mut print = String::from("1>");
            if self.args.unwrap().contains(&print) {
                let write_arr = self.args.unwrap().split_once("1>");
                self.args = write_arr.unwrap_or_default().0;
                self.file_path = Some(write_arr.unwrap_or_default().1.to_string());
                self.to_file = true;
            }
            let mut printfile = String::from(">");
            if self.args.unwrap().contains(&printfile) {
                let write_arr = com_arr.1.split_once("> ");
                self.args = write_arr.unwrap_or_default().0;
                self.file_path = Some(write_arr.unwrap_or_default().1.to_string());
                self.to_file = true;
            }
        }
        self.args = self.args
    }
    fn from_input_string(input: String) -> Self {
        input.trim();
        let tokens = get_tokens(&input);

        Self {
            to_file: false,
            type_of: CommandType::BuiltIn,
            file_path: None,
            execute: input,
            args: None,
            tokens: Vec::new(),
            arg: None,
        }

        // convert string to char array - or byte array if feeling zesty
        // function to get next param
        // pass word ending char to next word to word loop function
        // Check special actions at some stage (pipe output)
    }
    fn get_next_token(self, delimiter: char, input: String) {
        // if delimiter is single quotes, find next delimiter and return token
        if delimiter == '\'' {}
    }
    // if delimiter is white space
}

fn get_tokens(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let input_iter = input.chars().peekable();

    let new_token = String::new();

    // Some()

    tokens
}

// pub fn init_command(input: String) -> LineCommand {
//    input.trim();
//
//    let input_iter = input.chars().peekable();
//    let mut tokens: Vec<String> = Vec::new();
//
//    while input_iter.peek().is_some() {
//
//    }
//
// }

// pub fn get_next_token(delimiter: char, input: String) {
//     // if delimiter is single quotes, find next delimiter and return token
//     if delimiter == '\'' {
//         while self.arg.peek().is_some()? {
//
//         }
//     }
//     // if delimiter is white space
// }

pub fn get_type(command: &str) -> CommandType {
    if BUILTINS.contains(&command.trim()) {
        CommandType::BuiltIn
    } else if command.starts_with("'") || command.starts_with("\"") {
        CommandType::Absolute
    } else {
        CommandType::OnUserPATH
    }
}
