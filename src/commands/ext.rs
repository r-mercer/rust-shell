use std::env::{home_dir, set_current_dir};
// use std::error::Error;
// use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn cd(path: &str) {
    let mut dest = PathBuf::from(path);
    if dest.starts_with("~") {
        dest.strip_prefix("~").expect("cannot update path");
        dest = home_dir().expect("no home dir");
        set_current_dir(&dest).expect("No Home Dir");
    }
    // println!("Current passed path: {}", dest.to_string_lossy());
    // if dest.is_relative() {
    //     println!("{}", dest.to_string_lossy());
    //     let mut cwd = env::current_dir().expect("issue with current path");
    //     if dest.starts_with("./") {
    //         &dest.strip_prefix("./").expect("Issue with path prefix");
    //     }
    //     while let mut pre = dest.starts_with("../") {
    //         println!("Up a dir");
    //         dest.strip_prefix("../").expect("Issue with path prefix");
    //         cwd.pop();
    //         pre = dest.starts_with("../")
    //     }
    //     dest = dest.join(cwd);
    // }
    // dest.push(dest);
    let res = set_current_dir(dest);
    match res {
        Ok(()) => (),
        Err(e) => {
            let mut var = e.to_string();
            println!(
                "cd: {}: {}",
                path,
                var.get_mut(0..25).expect("err") // this just returns no such file....
            );
        }
    }
}

pub fn echo(str: &str) {
    let mut var = String::from(str);
    if var.contains("  ") && !var.starts_with("'") {
        let st = str.split_whitespace();
        var = st.map(|n| format!("{} ", n)).collect();
    }
    var = var.replace(['"', '\''], "");
    println!("{}", var.trim())
}

// pub fn echo(str: &str) {
//     let mut var = str.trim_start_matches('"').trim_start_matches('\'');
//     var = var.trim_end_matches('"').trim_end_matches('\'');
//     let mut var2 = var.replace("''", "").replace("''", "");
//     if var2.contains("  ") {
//         let st = var2.split_whitespace();
//         var2 = st.map(|n| format!("{} ", n)).collect();
//     }
//     println!("{}", var2.trim())
// }

pub fn cat(str: &str) {
    let mut list = Command::new("cat");
    let strs: Vec<&str> = str.split_inclusive(" '").collect();
    let strg = strs.iter().map(|f| f.replace(['"', '\''], ""));
    // let st = strg.split_whitespace();
    // list.arg(str).status().expect("file contents");
    list.args(strg).status().expect("file contents");
}

// fn get_path(path: &str) -> Result<PathBuf, Error> {
//     let mut dest = PathBuf::from(path);
//     if dest.is_relative() {
//         if dest.starts_with("./") {
//             dest.strip_prefix("./").expect("Issue with path prefix");
//         }
//         let mut cwd = env::current_dir()?;
//         while dest.starts_with("../") {
//             dest.strip_prefix("../").expect("Issue with path prefix");
//             cwd.pop();
//         }
//         dest = dest.join(cwd);
//     }
//     Ok(dest)
// }
// pub fn handle_path(mut pwd: PathBuf, dest: &str) -> PathBuf {
//     if dest_path.is_absolute() {
//         set_current_dir(dest_path);
//     } else if dest.starts_with("../") {
//         pwd.pop();
//     }
//     pwd.push(&dest);
//     pwd
// }
