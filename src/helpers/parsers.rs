static ESCAPES: [u8; 4] = [b'\\', b'\"', b'`', b' '];

pub fn get_tokens(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut input_iter = input.trim().bytes().enumerate().peekable();
    let input_chars: Vec<(usize, u8)> = input_iter.clone().collect();

    while input_iter.peek().is_some() {
        let delimiter: u8 = match input_iter.peek().unwrap().1 {
            b'\'' => {
                input_iter.next();
                b'\''
            }
            b'"' => {
                input_iter.next();
                b'"'
            }
            _ => b' ',
        };

        let parse_internal_quotes = |int: &usize, ch: &u8| -> bool {
            let mut in_singles_cl = false;
            let mut in_doubles_cl = false;

            if ch == &b'"' && input_chars[int - 1].1 != b'\\' {
                in_doubles_cl = !in_doubles_cl;
            } else if ch == &b'\'' && input_chars[int - 1].1 != b'\\' {
                in_singles_cl = !in_singles_cl;
            }
            if ch == &delimiter && !in_singles_cl && !in_doubles_cl {
                return delimiter != b' ' && input_chars[int + 1].1 != b' ';
            }
            true
        };

        if delimiter == b'\'' {
            let token_builder = input_iter.by_ref().take_while(|(_a, b)| b != &delimiter);
            let (_token_int_iter, token_iter): (Vec<usize>, Vec<u8>) = token_builder.unzip();
            let new_token = String::from_utf8(token_iter).unwrap_or_default();
            tokens.push(new_token);
        } else {
            let token_builder: Vec<(usize, u8)> = input_iter
                .by_ref()
                .take_while(|(a, b)| b != &delimiter && parse_internal_quotes(a, b))
                .collect();
            let token_iter = handle_escapes(token_builder);
            let new_token = String::from_utf8(token_iter).unwrap_or_default();
            tokens.push(new_token);
        }
    }
    tokens
}

fn handle_escapes(input: Vec<(usize, u8)>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for in_char in &input {
        if in_char.1 == b'\\' && ESCAPES.contains(&input[in_char.0 + 1].1) {
            break;
        } else {
            output.push(in_char.1);
        }
    }
    output
}
// fn parse_escapes(inp: )

// pub fn parse_comm(inp: &str) -> Vec<String> {
//     let mut bar = inp.chars().peekable();
//     let mut retvec: Vec<String> = Vec::new();
//     let mut fin = false;
//     let mut addword = false;
//
//     while !fin {
//         let mut retstr = String::new();
//         let mut withinquotes = false;
//         let mut withindoublequotes = false;
//
//         if addword {
//             retstr = retvec.pop().unwrap_or_default();
//             addword = false;
//         }
//
//         while bar.peek().is_some_and(|x| x.is_whitespace()) {
//             bar.next();
//         }
//
//         let ca: char = match bar.peek() {
//             Some('\'') => {
//                 bar.next();
//                 withinquotes = true;
//                 '\''
//             }
//             Some('"') => {
//                 bar.next();
//                 withindoublequotes = true;
//                 '"'
//             }
//             _ => ' ',
//         };
//
//         'wordloop: while bar.peek().is_some() {
//             let a: char = bar.next().unwrap();
//             if a == '\'' && !withindoublequotes {
//                 withinquotes = !withinquotes;
//             }
//             if a == '"' && !withinquotes {
//                 withindoublequotes = !withindoublequotes;
//             }
//             if a == '\\' {
//                 if withinquotes {
//                     retstr.push(a);
//                 } else if withindoublequotes {
//                     if bar.peek().is_some_and(|x| ESCAPES.contains(x)) {
//                         retstr.push(bar.next().unwrap());
//                     } else {
//                         retstr.push(a);
//                     }
//                 } else if bar.peek().is_some() {
//                     retstr.push(bar.next().unwrap());
//                 } else {
//                     retstr.push(a);
//                 }
//             } else if a == ca {
//                 if withinquotes {
//                     retstr.push(a);
//                 } else if bar.peek().is_some_and(|x| !x.is_whitespace()) && ca != ' ' {
//                     addword = true;
//                     break 'wordloop;
//                 } else if bar.peek().is_some_and(|x| x == &'/' || x == &'"') {
//                     bar.next();
//                     break 'wordloop;
//                 } else {
//                     break 'wordloop;
//                 }
//             } else if ca == ' ' && (a == '"' || a == '\'') {
//                 bar.next();
//             } else {
//                 retstr.push(a);
//             }
//         }
//         retvec.push(retstr);
//         fin = bar.size_hint().1.unwrap_or(0) == 0;
//     }
//     retvec
// }
