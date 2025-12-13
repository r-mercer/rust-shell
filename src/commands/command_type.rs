// use std::path::PathBuf;
use crate::commands::builtin::{exec_builtin, BUILTINS};
use crate::Command;

// static OUTPUTS: [&str; 2] = [">", "1>"];

pub enum StatusCode {
    Incomplete,
    Success,
    Failure,
    Exit,
}

pub enum OutputType {
    Str,
    Vec,
    None,
    // File,
}

pub struct ResultCode {
    pub status: StatusCode,
    pub output_type: OutputType,
    pub output_str: Option<String>,
    pub output_vec: Option<Vec<String>>,
}

impl ResultCode {
    pub fn from_result() -> Self {
        Self {
            status: StatusCode::Incomplete,
            output_type: OutputType::None,
            output_str: None,
            output_vec: None,
        }
    }
    pub fn from_none() -> Self {
        Self {
            status: StatusCode::Success,
            output_type: OutputType::None,
            output_str: None,
            output_vec: None,
        }
    }
    pub fn from_str(input: String) -> Self {
        Self {
            status: StatusCode::Success,
            output_type: OutputType::Str,
            output_str: Some(input),
            output_vec: None,
        }
    }
    pub fn from_vec(input: Vec<String>) -> Self {
        Self {
            status: StatusCode::Success,
            output_type: OutputType::Vec,
            output_str: None,
            output_vec: Some(input),
        }
    }
}

#[derive(Clone)]
pub enum CommandType {
    BuiltIn,
    OnUserPATH,
    Absolute,
}

#[derive(Clone)]
pub struct LineCommand {
    // pub to_file: bool,
    pub file_path: Option<String>,
    pub type_of: CommandType,
    pub executable: String,
    pub args: Option<Vec<String>>,
    pub params: Option<Vec<String>>,
    // pub result: Option<ResultCode>,
}

impl LineCommand {
    pub fn from_tokens_print(mut input: Vec<String>, mut out_input: Vec<String>) -> Self {
        let exe: String = input.drain(..1).collect();
        let params = input.extract_if(.., |x| x.starts_with("-")).collect();

        Self {
            type_of: get_type(&exe),
            file_path: out_input.pop(),
            executable: exe,
            args: Some(input),
            params: Some(params),
        }
    }

    pub fn from_tokens(mut input: Vec<String>) -> Self {
        let exe: String = input.drain(..1).collect();
        let params = input.extract_if(.., |x| x.starts_with("-")).collect();

        Self {
            type_of: get_type(&exe),
            file_path: None,
            executable: exe,
            args: Some(input),
            params: Some(params),
        }
    }

    pub fn vec_from_tokens(input: Vec<String>) -> Vec<LineCommand> {
        let mut ret_vec: Vec<LineCommand> = Vec::new();
        let mut command_iter = input.split(|x| x == "||").peekable();

        while command_iter.peek().is_some() {
            let vec = command_iter.next().unwrap();
            let int = vec.iter().position(|x| x == "1>" || x == ">");

            if let Some(int) = int {
                let (new_command, output) = vec.split_at(int);
                let com = LineCommand::from_tokens_print(new_command.to_vec(), output.to_vec());
                ret_vec.push(com);
            } else {
                let com = LineCommand::from_tokens(vec.to_vec());
                ret_vec.push(com);
            }
        }
        ret_vec
    }

    pub fn execute_command(&self) -> Result<ResultCode, ResultCode> {
        let mut return_result: ResultCode = ResultCode::from_result();

        if &self.executable == "exit" {
            return_result.status = StatusCode::Exit;
            return Ok(return_result);
        }

        match self.type_of {
            CommandType::BuiltIn => {
                let ret = exec_builtin(self);
                match ret {
                    Ok(res) => {
                        Ok(res)
                        // return_result.status = StatusCode::Success;
                        // return_result.output = res;
                        // Ok(return_result)
                    }
                    Err(err) => {
                        return_result.status = StatusCode::Failure;
                        return_result.output_str = Some(err.to_string());
                        Err(return_result)
                    }
                }
            }
            CommandType::OnUserPATH => {
                let ret = Command::new(&self.executable)
                    .args(self.args.as_ref().unwrap())
                    .status()
                    .unwrap_or_default();
                if ret.success() {
                    return_result.status = StatusCode::Success;
                    // return_result.output = ;
                    Ok(return_result)
                } else {
                    return_result.status = StatusCode::Failure;
                    Err(return_result)
                }
            }
            CommandType::Absolute => {
                let ret = Command::new(&self.executable)
                    .args(self.args.as_ref().unwrap())
                    .status()
                    .unwrap_or_default();
                if ret.success() {
                    return_result.status = StatusCode::Success;
                    Ok(return_result)
                } else {
                    return_result.status = StatusCode::Failure;
                    Err(return_result)
                }
            }
        }
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
