use std::fs;

pub fn to_file(path: String, contents: String) {
    let _ = fs::write(path, contents);
}
