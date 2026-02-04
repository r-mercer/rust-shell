use std::fs;
use std::io::{Result, Write};
use std::path::PathBuf;

use crate::commands::command_type::{LineCommand, ResultCode};

pub fn output_to(command: &LineCommand, result: &ResultCode) -> Result<()> {
    if let Some(path_str) = command.file_path.as_ref() {
        let path = PathBuf::from(path_str);
        fs::write(path, result.output_str.as_ref().unwrap().as_bytes())?;
    } else {
        if let Some(s) = result.output_str.as_ref() {
            println!("{}", s);
        }
    }
    Ok(())
}

pub fn output_to_vec(command: &LineCommand, result: &ResultCode) -> Result<()> {
    let mut buf = Vec::new();
    if let Some(output) = result.output_vec.as_ref() {
        for token in output {
            if command
                .params
                .as_ref()
                .is_some_and(|a| a.contains(&"-1".to_string()))
            {
                let _ = write!(buf, "{}\t", token);
            } else {
                let _ = writeln!(buf, "{}", token);
            }
        }
    }
    if let Some(path_str) = command.file_path.as_ref() {
        let path = PathBuf::from(path_str);
        fs::write(path, buf)?;
    } else {
        print!("{}", String::from_utf8_lossy(&buf));
    }
    Ok(())
}
