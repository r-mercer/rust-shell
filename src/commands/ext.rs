use std::env::{self};
use std::env::{home_dir, set_current_dir};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::{char, fs};

pub fn cd(path: String) -> Result<(), Error> {
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

pub fn print_ls(inp: Option<Vec<String>>) -> Result<String, Error> {
    // let ret = env::current_dir()?;
    // Ok(ret.display().to_string())

    let path = Path::new(&env::current_dir().unwrap_or_default());

    if inp.is_some() {
        path = Path::from(strs.first().unwrap());
    }

    let mut ret = String::new();
    let path = Path::new(strs.first().unwrap());

    for entry in fs::read_dir(path)? {
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

pub fn echo(strs: Vec<String>) -> Result<String, Error> {
    Ok(strs.join(" "))
}

pub fn cat(strs: Vec<String>) -> Result<String, Error> {
    // let mut list = Command::new("cat");
    // let strs: Vec<String> = parse_comm(str.trim());
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut ret = String::new();
    for path in strs {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        ret += &contents;
        ret.push(' ');
    }
    Ok(ret)
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
