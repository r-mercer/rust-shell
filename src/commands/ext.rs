use std::char;
use std::env::{home_dir, set_current_dir};
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
    let params = parse_comm(str);
    println!("{}", params.join(" "));
}

pub fn cat(str: &str) {
    let mut list = Command::new("cat");
    let strs: Vec<String> = parse_comm(str.trim());
    list.args(strs).status().expect("file contents");
}

pub fn parse_comm(inp: &str) -> Vec<String> {
    let mut bar = inp.chars().peekable();
    let mut retvec: Vec<String> = Vec::new();
    let mut fin = false;
    let mut addword = false;

    // let ca: char = match bar.peek().filter(|x: char| !x.is_whitespace()) {
    while !fin {
        let mut retstr = String::new();
        let mut withinquotes = false;
        let mut withindoublequotes = false;

        if addword {
            retstr = retvec.pop().unwrap_or_default();
        }

        while bar.peek().is_some_and(|x| x == &' ') {
            bar.next();
        }

        let ca: char = match bar.peek() {
            Some('\'') => {
                bar.next();
                withinquotes = true;
                // println!("withinquotes: {}", withinquotes);
                '\''
            }
            Some('"') => {
                bar.next();
                withindoublequotes = true;
                '"'
            }
            _ => ' ',
        };
        'wordloop: while bar.peek().is_some() {
            // println!("withinquotes: {}", withinquotes);
            let a: char = bar.next().unwrap();
            // println!("ca: {} | a: {}", ca, a);
            // if a == '\'' && bar.peek().is_some_and(|x| !ESCAPES.contains(&x)) {
            // if a == '\'' && !retstr.ends_with('\\') {
            if a == '\'' && !withindoublequotes {
                withinquotes = !withinquotes;
            }
            if a == '"' && !withinquotes {
                withindoublequotes = !withindoublequotes;
            }
            // println!("withinquotes: {}", withinquotes);
            // println!("withindoublequotes: {}", withindoublequotes);
            if a == '\\' {
                if withinquotes {
                    retstr.push(a);
                } else if withindoublequotes {
                    if bar.peek().is_some_and(|x| ESCAPES.contains(x)) {
                        retstr.push(bar.next().unwrap());
                    // println!("esc at 81,{}", a);
                    } else {
                        retstr.push(a);
                    }
                } else if bar.peek().is_some() {
                    retstr.push(bar.next().unwrap());
                } else {
                    // break 'wordloop;
                    retstr.push(a);
                    // println!("esc at 89,{}", a);
                }
            } else if a == ca && bar.peek().is_none() {
                break 'wordloop;
            } else if a == ca {
                if withinquotes {
                    retstr.push(a);
                    // println!("line:{}", 108);
                } else if bar.peek().is_some_and(|x| !x.is_whitespace()) && ca != ' ' {
                    // println!("add wordline:{}", 114);
                    addword = true;
                    break 'wordloop;
                } else if bar.peek().is_some_and(|x| x == &'/' || x == &'"') && ca != ' ' {
                    bar.next();
                    // println!("line:{}", 111);
                    break 'wordloop;
                    // retstr.push(bar.next().unwrap());
                } else {
                    // println!("line:{}", 111);
                    break 'wordloop;
                }
            } else {
                retstr.push(a);
            }
        }
        // if addword {
        //     push_last(retvec.pop().unwrap_or_default(), retstr);
        // } else {
        //     retvec.push(retstr);
        // }
        retvec.push(retstr);
        fin = bar.size_hint().1.unwrap_or(0) == 0;
    }
    retvec
}

// fn push_last(mut pop: String, push: String) -> String {
//     pop += push.as_str();
//     pop
// }

static ESCAPES: [char; 4] = ['\\', '\"', '`', ' '];

#[allow(dead_code)]
pub fn echo_test(str: &str) -> String {
    let params = parse_comm(str);
    params.join(" ")
    // println!("{}", params.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escapes() {
        // assert_eq!(echo_test(r#"hello'shell'\\'test"#), r#"hello'shell'\'test"#);
        // assert_eq!(
        //     echo_test(r#"hello\"insidequotes"test\"#),
        //     r#"hello"insidequotestest""#
        // );
        assert_eq!(
            echo_test(r#""script  world"  "test""example""#),
            r#"script  world testexample"#
        );
        assert_eq!(echo_test(r#"/tmp/owl/'f \58\'"#), r#"/tmp/owl/'f \58\'"#);
        assert_eq!(echo_test(r#"/tmp/bee/'f \96\'"#), r#"/tmp/bee/'f \96\'"#);
        assert_eq!(echo_test(r#"\'\"shell hello\"\'"#), r#"'"shell hello"'"#);
        assert_eq!(echo_test(r#"shell\nexample"#), r#"shellnexample"#);
        assert_eq!(
            echo_test(r#"script'test'\\'world"#),
            r#"script'test'\'world"#
        );

        assert_eq!(echo_test(r#""/tmp/bee/f \51\'""#), r#"/tmp/bee/f \51\'"#);
    }
}
// fn replace_escape(mut par: &str) -> String {
//     let mut retstr = String::new();
//     let retstr = par.escape_unicode().collect();
//     // retstr = par.replace("\"", )
//     retstr
// }

// fn get_next_param(mut par: &str) -> (String, &str) {
//     let c: char = match par.trim().get(0..1) {
//         Some("'") => '\'',
//         Some("\"") => '"',
//         _ => ' ',
//     };
//     // let str = String::from("[^\\]").push_str('"' + c);
//     // let re = Regex::new(format!("[^\\\\]{}", c).as_str()).expect("Invalid regex pattern");
//     par = par.strip_prefix(c).unwrap_or(par);
//     let mut ind = par.len();
//
//     if par.contains('\'') || par.contains('\"') {
//         if c == '\'' || c == '"' {
//             par.find("\"").unwrap_or(par.find("\'").unwrap_or(ind));
//         }
//     } else {
//         ind = par.find(c).unwrap_or(par.len());
//     }
//     let (a, b) = par.split_at(ind);
//     let mut para = check_escape(a, c);
//     para = para.replace("\"\"", "").replace("''", "");
//     let mut parb = b.strip_prefix(c).unwrap_or(b);
//
//     if parb.starts_with(['\'', '"']) {
//         let (d, e) = get_next_param(parb);
//         para += d.as_str();
//         parb = e;
//     }
//     (para, parb)
// }
//
// fn check_escape(a: &str, c: char) -> String {
//     let para = String::from(a);
//     match c {
//         '"' => para,
//         '\'' => para,
//         _ => para.replace('\\', ""),
//     }
// }
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
