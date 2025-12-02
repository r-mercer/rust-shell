use std::env::{self};
use std::env::{home_dir, set_current_dir};
use std::io::Error;
use std::path::PathBuf;
use std::{char, fs};

pub fn cd(path: Option<Vec<String>>) -> Result<(), Error> {
    let mut dest;

    if let Some(path_string) = path {
        dest = PathBuf::from(path_string.concat());
    } else {
        dest = env::current_dir()?;
    }

    if dest.starts_with("~") {
        dest.strip_prefix("~").expect("cannot update path");
        dest = home_dir().expect("no home dir");
        set_current_dir(&dest).expect("No Home Dir");
    }

    set_current_dir(dest)
}

pub fn print_ls(inp: Option<Vec<String>>) -> Result<String, Error> {
    let mut path_string = env::current_dir().unwrap_or_default();
    if let Some(prepath) = inp {
        path_string = PathBuf::from(prepath[0].clone());
    }

    let mut ret = String::new();

    for entry in fs::read_dir(path_string)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            println!("  [DIR] {}", path.display());
            ret.push_str(path.to_str().unwrap_or_default());
        } else if path.is_file() {
            ret.push_str(path.to_str().unwrap_or_default());
            println!("  [FILE] {}", path.display());
        } else {
            ret.push_str(path.to_str().unwrap_or_default());
            println!("  [OTHER] {}", path.display());
        }
    }
    Ok(ret)
}

pub fn print_wd() -> Result<String, Error> {
    let ret = env::current_dir()?;
    Ok(ret.display().to_string())
}

pub fn echo(strs: Option<Vec<String>>) -> Result<String, Error> {
    if let Some(paths) = strs {
        Ok(paths.join(" "))
    } else {
        Ok(String::from(""))
    }
}

pub fn cat(strs: Option<Vec<String>>) -> Result<String, Error> {
    // let mut list = Command::new("cat");
    // let strs: Vec<String> = parse_comm(str.trim());
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut ret = String::new();
    for path in strs.unwrap_or_default() {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        ret += &contents;
        ret.push(' ');
    }
    Ok(ret)
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
