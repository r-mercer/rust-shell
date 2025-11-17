use std::char;
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
    let params = parse_param(str);
    // if var.contains("  ") && !var.starts_with("'") {
    //     let st = str.split_whitespace();
    //     var = st.map(|n| format!("{} ", n)).collect();
    // }
    // var = var.replace(['"', '\''], "");
    println!("strs curr leg: {}", params.len());
    for par in params {
        println!("{}", par)
    }
}

pub fn cat(str: &str) {
    let mut list = Command::new("cat");
    // println!("{}", str);
    // let strs: Vec<&str> = str.split_inclusive(" '").collect();
    let strs: Vec<&str> = parse_param(str);
    // str.split_whitespace().collect();
    // // let strs = strs.iter().collect::<Vec<&str>>();

    // let strg = strs.iter().map(|f| f.replace(['"', '\''], ""));
    // let st = strg.split_whitespace();
    // list.arg(str).status().expect("file contents");
    list.args(strs).status().expect("file contents");
}

pub fn parse_param(mut par: &str) -> Vec<&str> {
    let mut strs: Vec<&str> = Vec::new();
    par = par.trim();
    while !par.is_empty() {
        let c: char = match par.get(0..1) {
            Some("'") => '\'',
            Some("\"") => '"',
            _ => ' ',
        };
        println!("c curr val: {}", c);
        println!("strs curr leg: {}", strs.len());
        par = par.trim().strip_prefix(c).unwrap_or(par);
        let ind = par.find(c).unwrap_or(par.len());
        let (a, b) = par.split_at(ind);
        strs.push(a);
        par = b.strip_prefix(c).unwrap_or(b).trim();
    }
    strs
}
// fn get_pars(mut pars: )
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
