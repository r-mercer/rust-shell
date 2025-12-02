use std::fs::File;
use std::io::{Result, Write};

pub fn to_file(path: String, contents: String) -> Result<()> {
    let mut file = File::create(path.trim())?;
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}
