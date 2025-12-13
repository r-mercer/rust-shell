use std::fs;
use std::io::{Result, Write};
use std::path::PathBuf;

use crate::commands::command_type::{LineCommand, ResultCode};

pub fn output_to(command: &LineCommand, result: &ResultCode) -> Result<()> {
    let path = PathBuf::from(command.file_path.as_ref().unwrap());
    fs::write(path, result.output_str.as_ref().unwrap().as_bytes())?;
    Ok(())
}

pub fn output_to_vec(command: &LineCommand, result: &ResultCode) -> Result<()> {
    let path = PathBuf::from(command.file_path.as_ref().unwrap());
    let mut buf = Vec::new();

    // this should be if param is -1 then concat with tab
    if let Some(output) = result.output_vec.as_ref() {
        for token in output {
            if command
                .params
                .as_ref()
                .is_some_and(|a| a.contains(&"-1".to_string()))
            {
                let _ = write!(buf, "{:?}", token);
            } else {
                let _ = writeln!(buf, "{:?}", token);
            }
        }
    }
    if command.file_path.is_some() {
        fs::write(path, buf)?;
    } else {
        print!("{:?}", buf);
    }
    // this could be print or write

    Ok(())
}
