use std::char;
use std::env::{home_dir, set_current_dir};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn cd(path: &str) -> Result<(), Error> {
    let mut dest = PathBuf::from(path);
    if dest.starts_with("~") {
        dest.strip_prefix("~").expect("cannot update path");
        dest = home_dir().expect("no home dir");
        set_current_dir(&dest).expect("No Home Dir");
    }

    set_current_dir(dest)

    // let res = set_current_dir(dest);
    // match res {
    //     Ok(()) => (),
    //     Err(e) => {
    //         let mut var = e.to_string();
    //         println!(
    //             "cd: {}: {}",
    //             path,
    //             var.get_mut(0..25).expect("err") // this just returns no such file....
    //         );
    //     }
    // }
}

pub fn echo(str: &str) -> Result<String, Error> {
    let params = parse_comm(str);
    // println!("{}", params.join(" "));
    Ok(params.join(" "))
}

pub fn cat(str: &str) -> Result<String, Error> {
    let mut list = Command::new("cat");
    let strs: Vec<String> = parse_comm(str.trim());
    let status = list.args(strs).status();
    status.to_string()
}

pub fn parse_comm(inp: &str) -> Vec<String> {
    let mut bar = inp.chars().peekable();
    let mut retvec: Vec<String> = Vec::new();
    let mut fin = false;
    let mut addword = false;

    while !fin {
        let mut retstr = String::new();
        let mut withinquotes = false;
        let mut withindoublequotes = false;

        if addword {
            retstr = retvec.pop().unwrap_or_default();
            addword = false;
        }

        while bar.peek().is_some_and(|x| x.is_whitespace()) {
            bar.next();
        }

        let ca: char = match bar.peek() {
            Some('\'') => {
                bar.next();
                withinquotes = true;
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
            let a: char = bar.next().unwrap();
            if a == '\'' && !withindoublequotes {
                withinquotes = !withinquotes;
            }
            if a == '"' && !withinquotes {
                withindoublequotes = !withindoublequotes;
            }
            // println!(
            //     "ca:{} | a:{} | wq: {} | wdq: {} |aw:{}",
            //     ca, a, withinquotes, withindoublequotes, addword
            // );
            if a == '\\' {
                if withinquotes {
                    retstr.push(a);
                } else if withindoublequotes {
                    if bar.peek().is_some_and(|x| ESCAPES.contains(x)) {
                        retstr.push(bar.next().unwrap());
                    } else {
                        retstr.push(a);
                    }
                } else if bar.peek().is_some() {
                    retstr.push(bar.next().unwrap());
                } else {
                    retstr.push(a);
                }
            } else if a == ca {
                if withinquotes {
                    retstr.push(a);
                } else if bar.peek().is_some_and(|x| !x.is_whitespace()) && ca != ' ' {
                    addword = true;
                    break 'wordloop;
                } else if bar.peek().is_some_and(|x| x == &'/' || x == &'"') {
                    bar.next();
                    break 'wordloop;
                } else {
                    break 'wordloop;
                }
            } else if ca == ' ' && (a == '"' || a == '\'') {
                bar.next();
            } else {
                retstr.push(a);
            }
        }
        retvec.push(retstr);
        fin = bar.size_hint().1.unwrap_or(0) == 0;
    }
    retvec
}

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
        assert_eq!(
            echo_test(r#""script  world" "test""example""#),
            r#"script  world testexample"#
        );
        assert_eq!(echo_test(r#"\'\"shell hello\"\'"#), r#"'"shell hello"'"#);
        assert_eq!(echo_test(r#"shell\nexample"#), r#"shellnexample"#);

        assert_eq!(echo_test(r#""/tmp/bee/f \51\'""#), r#"/tmp/bee/f \51\'"#);
    }
}
