static ESCAPES: [char; 4] = ['\\', '\"', '`', ' '];

// Handle Escapes when being added to token vec
// Handle expansions at a different time?

pub fn get_tokens(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut input_iter = input.chars().peekable();
    let input_chars: Vec<char> = input_iter.clone().collect();
    let mut interval = 0;

    while input_iter.peek().is_some() {
        let delimiter: char = match input_iter.peek().unwrap() {
            '\'' => '\'',
            '"' => '"',
            _ => ' ',
        };
        println!("delimiter:{}", delimiter);
        // let token: Vec<(usize, char)> = Vec::new();
        let mut new_token = String::new();

        let parse_internal_quotes = |int: &usize, ch: &char| -> bool {
            let mut in_singles_cl = false;
            let mut in_doubles_cl = false;

            if ch == &'"' && input_chars[int - 1] != '\\' {
                in_doubles_cl = !in_doubles_cl;
            } else if ch == &'\'' && input_chars[int - 1] != '\\' {
                in_singles_cl = !in_singles_cl;
            }
            if ch == &delimiter && !in_singles_cl && !in_doubles_cl {
                return true;
            }
            false
        };

        let mut token_iter: Vec<char> = Vec::new();

        if delimiter == '\'' {
            token_iter = input_iter
                .by_ref()
                .take_while(|a| a != &delimiter)
                .collect();
        } else {
            let token_builder = input_iter
                .by_ref()
                .enumerate()
                .take_while(|(a, b)| b != &delimiter && parse_internal_quotes(a, b))
                .collect();
            token_iter = handle_escapes(token_builder);
        }

        // for t in token_iter {
        let new_item = token_iter.display();
        new_token.push(token_iter.concat());
        // }
        interval += 1;
        println!("new_token:{}", new_token);
        tokens.push(new_token);

        if interval == 5 {
            break;
        }
    }
    tokens
}

fn handle_escapes(input: Vec<(usize, char)>) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    for in_char in &input {
        if in_char.1 == '\\' && ESCAPES.contains(&input[in_char.0 + 1].1) {
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
