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

        while let Some(vec) = command_iter.next() {
            // Improved: support for '> file' after command (ignore '1>' for now, as not supported)
            let mut i = 0;
            let mut file_path: Option<String> = None;
            let mut cmd_tokens: Vec<String> = Vec::new();
            while i < vec.len() {
                if vec[i] == ">" {
                    // Next token is output file if present
                    if i + 1 < vec.len() {
                        file_path = Some(vec[i + 1].clone());
                        i += 2;
                        // Skip extra > and file tokens (only handle one redirection)
                        break;
                    } else {
                        i += 1;
                    }
                } else {
                    cmd_tokens.push(vec[i].clone());
                    i += 1;
                }
            }
            // Collect any trailing tokens as they may be part of the file path (bad syntax)
            let mut line_command = if !cmd_tokens.is_empty() {
                LineCommand::from_tokens(cmd_tokens)
            } else {
                // fallback: treat as empty command (shouldn't occur)
                LineCommand::from_tokens(vec![])
            };
            line_command.file_path = file_path;
            ret_vec.push(line_command);
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
                use std::process::Stdio;
                let mut cmd = Command::new(&self.executable);
                if let Some(args) = self.args.as_ref() {
                    cmd.args(args);
                }
                if let Some(path) = self.file_path.as_ref() {
                    // Redirect output to file
                    match std::fs::File::create(path) {
                        Ok(file) => {
                            let child = cmd.stdout(Stdio::from(file)).status();
                            match child {
                                Ok(status) if status.success() => {
                                    return_result.status = StatusCode::Success;
                                    Ok(return_result)
                                }
                                Ok(_) | Err(_) => {
                                    return_result.status = StatusCode::Failure;
                                    Err(return_result)
                                }
                            }
                        }
                        Err(e) => {
                            return_result.status = StatusCode::Failure;
                            return_result.output_str =
                                Some(format!("Redirection file error: {}", e));
                            Err(return_result)
                        }
                    }
                } else {
                    let ret = cmd.status().unwrap_or_default();
                    if ret.success() {
                        return_result.status = StatusCode::Success;
                        Ok(return_result)
                    } else {
                        return_result.status = StatusCode::Failure;
                        Err(return_result)
                    }
                }
            }
            CommandType::Absolute => {
                use std::process::Stdio;
                let mut cmd = Command::new(&self.executable);
                if let Some(args) = self.args.as_ref() {
                    cmd.args(args);
                }
                if let Some(path) = self.file_path.as_ref() {
                    // Redirect output to file
                    match std::fs::File::create(path) {
                        Ok(file) => {
                            let child = cmd.stdout(Stdio::from(file)).status();
                            match child {
                                Ok(status) if status.success() => {
                                    return_result.status = StatusCode::Success;
                                    Ok(return_result)
                                }
                                Ok(_) | Err(_) => {
                                    return_result.status = StatusCode::Failure;
                                    Err(return_result)
                                }
                            }
                        }
                        Err(e) => {
                            return_result.status = StatusCode::Failure;
                            return_result.output_str =
                                Some(format!("Redirection file error: {}", e));
                            Err(return_result)
                        }
                    }
                } else {
                    let ret = cmd.status().unwrap_or_default();
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
