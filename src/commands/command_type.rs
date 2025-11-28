enum CommandType {
    BuiltIn,
    BuiltInSilent,
    OnUserPATH,
    Absolute,
}

struct LineCommand {
    to_print: bool,
    type_of: CommandType,
    execute: String,
}

pub fn get_type(command: &str) {}
