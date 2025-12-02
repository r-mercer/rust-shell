static ESCAPES: [char; 4] = ['\\', '\"', '`', ' '];

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
