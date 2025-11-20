// use regex::{Match, Regex};
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
    // println!("echo string passed: {}", str);
    let params = parse_comm(str);
    println!("{}", params.join(" "));
}

pub fn cat(str: &str) {
    let mut list = Command::new("cat");
    let strs: Vec<String> = parse_param(str.trim());
    list.args(strs).status().expect("file contents");
}

pub fn parse_param(mut par: &str) -> Vec<String> {
    let mut retvec: Vec<String> = Vec::new();

    // println!("parse param passed: {}", par);
    while !par.is_empty() {
        let (a, b) = get_next_param(par);
        retvec.push(a);
        par = b.trim();
    }
    retvec
}

pub fn parse_comm(inp: &str) -> Vec<String> {
    let mut bar = inp.chars().peekable();
    let mut retvec: Vec<String> = Vec::new();
    let mut fin = false;
    // println!("in lines: {}", inp);

    // 'stringloop: while !fin {
    while !fin {
        let mut retstr = String::new();
        let ca: char = match bar.peek() {
            Some('\'') => {
                bar.next();
                '\''
            }
            Some('"') => {
                bar.next();
                '"'
            }
            _ => ' ',
        };
        // println!("ca: <{}>", ca);
        'wordloop: while bar.peek().is_some() {
            let a: char = bar.next().unwrap_or(' ');
            // println!("a: <{}>", a);
            if a == '\\' {
                if bar.peek().is_some() {
                    retstr.push(bar.next().unwrap());
                } else {
                    break 'wordloop;
                }
            } else if a == ca {
                break 'wordloop;
            } else {
                retstr.push(a);
            }
        }
        retvec.push(retstr);
        fin = bar.size_hint().1.unwrap_or(0) == 0;
    }
    retvec
}
// fn replace_escape(mut par: &str) -> String {
//     let mut retstr = String::new();
//     let retstr = par.escape_unicode().collect();
//     // retstr = par.replace("\"", )
//     retstr
// }

fn get_next_param(mut par: &str) -> (String, &str) {
    let c: char = match par.trim().get(0..1) {
        Some("'") => '\'',
        Some("\"") => '"',
        _ => ' ',
    };
    // let str = String::from("[^\\]").push_str('"' + c);
    // let re = Regex::new(format!("[^\\\\]{}", c).as_str()).expect("Invalid regex pattern");
    par = par.strip_prefix(c).unwrap_or(par);
    let mut ind = par.len();

    if par.contains('\'') || par.contains('\"') {
        if c == '\'' || c == '"' {
            par.find("\"").unwrap_or(par.find("\'").unwrap_or(ind));
        }
    } else {
        ind = par.find(c).unwrap_or(par.len());
    }
    let (a, b) = par.split_at(ind);
    let mut para = check_escape(a, c);
    para = para.replace("\"\"", "").replace("''", "");
    let mut parb = b.strip_prefix(c).unwrap_or(b);

    if parb.starts_with(['\'', '"']) {
        let (d, e) = get_next_param(parb);
        para += d.as_str();
        parb = e;
    }
    (para, parb)
}

fn check_escape(a: &str, c: char) -> String {
    let para = String::from(a);
    match c {
        '"' => para,
        '\'' => para,
        _ => para.replace('\\', ""),
    }
}
//     let c: char = match par.get(0..1) {
//         Some("'") => '\'',
//         Some("\"") => '"',
//         _ => ' ',
//     };
//     par = par.strip_prefix(c).unwrap_or(par);
//     let ind = par.find(c).unwrap_or(par.len());
//     let (ret, b) = par.split_at(ind);
//     par = b.strip_prefix(c).unwrap_or(b);
//     if
//     (ret, par)
// }
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
