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

pub fn get_type(command: &str) -> CommandType {
    if BUILTINS.contains(&command.trim()) {
        CommandType::BuiltIn
    } else if command.starts_with("'") || command.starts_with("\"") {
        CommandType::Absolute
    } else {
        CommandType::OnUserPATH
    }
}
