use std::env::{self, set_current_dir};
use std::path::PathBuf;

pub fn cd(path: &str) {
    let mut cwd = env::current_dir().expect("cannot find dir");
    let dest = PathBuf::from(path);
    if dest.is_relative() {
        while dest.starts_with("../") {
            dest.strip_prefix("../");
            cwd.pop();
        }
        cwd.push(dest);
        set_current_dir(cwd);
        return;
    }
    env::set_current_dir(dest);
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
