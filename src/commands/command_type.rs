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
    pub arg: Option<String>,
}

impl LineCommand {
    fn check_output(self) {
        if self.args.is_some() {
            let mut print = String::from("1>");
            if self.args.unwrap().contains(&print) {
                let 
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
}

pub fn get_type(command: &str) -> CommandType {
    if BUILTINS.contains(&command.trim()) {
        CommandType::BuiltIn
    } else if command.starts_with("'") || command.starts_with("\"") {
        CommandType::Absolute
    } else {
        CommandType::OnUserPATH
    }
}
