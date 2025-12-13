use std::env::{self};
use std::env::{home_dir, set_current_dir};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::commands::command_type::{LineCommand, ResultCode};

// pub fn cd(path: &Option<Vec<String>>) -> Result<ResultCode, Error> {
pub fn cd(command: &LineCommand) -> Result<ResultCode, Error> {
    let mut dest;

    if let Some(path_string) = &command.args {
        dest = PathBuf::from(path_string.concat());
    } else {
        dest = env::current_dir()?;
    }

    if dest.starts_with("~") {
        dest.strip_prefix("~").expect("cannot update path");
        dest = home_dir().expect("no home dir");
        set_current_dir(&dest).expect("No Home Dir");
    }
    set_current_dir(dest)?;
    Ok(ResultCode::from_none())
}

pub fn print_ls(command: &LineCommand) -> Result<ResultCode, Error> {
    let mut path_string = env::current_dir().unwrap_or_default();
    if let Some(prepath) = command.args.to_owned() {
        if !prepath.is_empty() {
            path_string = PathBuf::from(prepath[0].clone());
        }
    }

    let mut ret_vec: Vec<String> = Vec::new();

    for entry in fs::read_dir(path_string)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // println!("  [DIR] {}", path.display());
            ret_vec.push(path.to_str().unwrap_or_default().to_string());
        } else if path.is_file() {
            // println!("  [FILE] {}", path.display());
            ret_vec.push(path.to_str().unwrap_or_default().to_string());
        } else {
            // println!("  [OTHER] {}", path.display());
            ret_vec.push(path.to_str().unwrap_or_default().to_string());
        }
    }
    Ok(ResultCode::from_vec(ret_vec))
}

pub fn print_wd() -> Result<ResultCode, Error> {
    let ret = env::current_dir()?;
    Ok(ResultCode::from_str(ret.display().to_string()))
}

pub fn echo(command: &LineCommand) -> Result<ResultCode, Error> {
    if let Some(paths) = &command.args {
        Ok(ResultCode::from_vec(paths.to_vec()))
    } else {
        Ok(ResultCode::from_none())
    }
}

pub fn cat(command: &LineCommand) -> Result<ResultCode, Error> {
    let mut ret_vec = Vec::new();
    for path in command.args.clone().unwrap_or_default() {
        ret_vec.push(fs::read_to_string(path).expect("Should have been able to read the file"));
    }
    Ok(ResultCode::from_vec(ret_vec))
}

// #[allow(dead_code)]
// pub fn echo_test(str: &str) -> String {
//     let params = handlers::parse_comm(str);
//     params.join(" ")
//     // println!("{}", params.join(" "));
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_escapes() {
//         assert_eq!(
//             echo_test(r#""script  world" "test""example""#),
//             r#"script  world testexample"#
//         );
//         assert_eq!(echo_test(r#"\'\"shell hello\"\'"#), r#"'"shell hello"'"#);
//         assert_eq!(echo_test(r#"shell\nexample"#), r#"shellnexample"#);
//
//         assert_eq!(echo_test(r#""/tmp/bee/f \51\'""#), r#"/tmp/bee/f \51\'"#);
//     }
// }
