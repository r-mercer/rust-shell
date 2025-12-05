static ESCAPES: [char; 4] = ['\\', '\"', '`', ' '];

pub fn get_tokens(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: Vec<(usize, char)> = Vec::new();

    let input_iter = input.char_indices().peekable();
    let delimiter: char = ' ';

    let parse_internal_quotes = |int: &usize, ch: &char| -> bool {
        let mut in_singles_cl = false;
        let mut in_doubles_cl = false;

        if ch == &'"' && token[int - 1].1 != '\\' {
            in_doubles_cl = !in_doubles_cl;
        } else if ch == &'\'' && token[int - 1].1 != '\\' {
            in_singles_cl = !in_singles_cl;
        }
        if ch == &delimiter && !in_singles_cl && !in_doubles_cl {
            return true;
        }
        false
    };

    if delimiter == '\'' {
        token = input_iter.take_while(|(_i, a)| a != &delimiter).collect();
    } else if delimiter == '"' {
        token = input_iter
            .take_while(|(i, a)| a != &delimiter && parse_internal_quotes(i, a))
            .filter(|(j, b)| b == &'\\' && ESCAPES.contains(&token[j + 1].1))
            .collect();
    }
    let mut new_token = String::new();
    for (_k, c) in token {
        new_token.push(c);
    }
    tokens.push(new_token);
    // for tok in token {
    //     tokens.push(tok);
    // }
    tokens
}

// fn parse_escapes(inp: )

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
