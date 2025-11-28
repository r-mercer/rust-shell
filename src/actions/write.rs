use std::{fs, path};

pub fn to_file(path: String, contents: String) {
    fs::write(path, contents);
}
