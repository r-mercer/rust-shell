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
    pub to_file: bool,
    pub file_path: Option<String>,
    pub type_of: CommandType,
    pub executable: String,
    pub args: Option<Vec<String>>,
}

impl LineCommand {
    // fn check_output(self) {
    //     if self.args.is_some() {
    //         let mut print = String::from("1>");
    //         if self.args.unwrap().contains(&print) {
    //             let write_arr = self.args.unwrap().split_once("1>");
    //             self.args = write_arr.unwrap_or_default().0;
    //             self.file_path = Some(write_arr.unwrap_or_default().1.to_string());
    //             self.to_file = true;
    //         }
    //         let mut printfile = String::from(">");
    //         if self.args.unwrap().contains(&printfile) {
    //             let write_arr = com_arr.1.split_once("> ");
    //             self.args = write_arr.unwrap_or_default().0;
    //             self.file_path = Some(write_arr.unwrap_or_default().1.to_string());
    //             self.to_file = true;
    //         }
    //     }
    //     self.args = self.args
    // }
    // pub fn from_many_tokens(mut input: Vec<String>) -> Vec<Self> {}
    // pub fn from_tokens_w_output(mut input: Vec<String>, output: bool) -> Self {
    //     let exe: String = input.drain(..1).collect();
    //     // if input.contains(OUTPUTS)
    //     let inter = input.iter().position(|x| OUTPUTS.contains(&x.as_str()));
    //     if let Some(inter) = int {
    //         let write_to = input.partition_point(|x| OUTPUTS.contains(&x.as_str()));
    //         file_path =
    //     }
    //
    //     Self {
    //         to_file: output,
    //         type_of: get_type(&exe),
    //         file_path: None,
    //         executable: exe,
    //         args: Some(input),
    //     }
    // }

    pub fn from_tokens_print(mut input: Vec<String>, mut out_input: Vec<String>) -> Self {
        let exe: String = input.drain(..1).collect();

        Self {
            to_file: true,
            type_of: get_type(&exe),
            file_path: out_input.pop(),
            executable: exe,
            args: Some(input),
        }
    }

    pub fn from_tokens(mut input: Vec<String>) -> Self {
        let exe: String = input.drain(..1).collect();

        Self {
            to_file: false,
            type_of: get_type(&exe),
            file_path: None,
            executable: exe,
            args: Some(input),
        }
    }

    pub fn vec_from_tokens(input: Vec<String>) -> Vec<LineCommand> {
        let mut ret_vec: Vec<LineCommand> = Vec::new();

        let mut command_iter = input.split(|x| x == "||").peekable();

        while command_iter.peek().is_some() {
            let vec = command_iter.next().unwrap();
            let int = vec.iter().position(|x| x == "1>");
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

    // pub fn check_output() {}

    pub fn execute_command(&self) -> Result<ResultCode, ResultCode> {
        // println!("execute:{}", &self.executable);
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

    // fn get_next_token(self, delimiter: char, input: String) {
    //     // if delimiter is single quotes, find next delimiter and return token
    //     if delimiter == '\'' {}
    // }
    // if delimiter is white space
}

// fn get_tokens(input: &str) -> Vec<String> {
//     let mut tokens: Vec<String> = Vec::new();
//     let input_iter = input.chars().peekable();
//
//     let new_token = String::new();
//
//     // Some()
//
//     tokens
// }

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
