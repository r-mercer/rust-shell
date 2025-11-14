use std::env::{self, set_current_dir};
// use std::error::Error;
use std::io::Error;
use std::path::PathBuf;

pub fn cd(path: &str) {
    let mut dest = PathBuf::from(path);
    if dest.is_relative() {
        let mut cwd = env::current_dir().expect("issue with current path");
        while dest.starts_with("../") {
            dest.strip_prefix("../").expect("Issue with path prefix");
            cwd.pop();
        }
        dest = dest.join(cwd);
    }
    let res = set_current_dir(dest);
    match res {
        Ok(y) => (),
        Err(e) => println!("{}: No such file or directory", path),
    }
}

fn get_path(path: &str) -> Result<PathBuf, Error> {
    let mut dest = PathBuf::from(path);
    if dest.is_relative() {
        let mut cwd = env::current_dir()?;
        while dest.starts_with("../") {
            dest.strip_prefix("../").expect("Issue with path prefix");
            cwd.pop();
        }
        dest = dest.join(cwd);
    }
    Ok(dest)
}
// pub fn handle_path(mut pwd: PathBuf, dest: &str) -> PathBuf {
//     if dest_path.is_absolute() {
//         set_current_dir(dest_path);
//     } else if dest.starts_with("../") {
//         pwd.pop();
//     }
//     pwd.push(&dest);
//     pwd
// }
