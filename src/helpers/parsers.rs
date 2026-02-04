pub fn get_tokens(input: String) -> Vec<String> {
    // Improved: respects spaces, quoted substrings, and > operators for redirection
    let mut tokens = Vec::new();
    let mut chars = input.trim().chars().peekable();
    let mut buf = String::new();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '\'' if !in_double => {
                in_single = !in_single;
                buf.push(ch);
                chars.next();
            }
            '"' if !in_single => {
                in_double = !in_double;
                buf.push(ch);
                chars.next();
            }
            '>' if !in_single && !in_double => {
                // Push buffer if any
                if !buf.trim().is_empty() {
                    tokens.push(buf.trim().to_string());
                    buf.clear();
                }
                // Consume > and any whitespace
                chars.next();
                tokens.push(" > ".trim().to_string());
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
            }
            ' ' if !in_single && !in_double => {
                if !buf.is_empty() {
                    tokens.push(buf.trim().to_string());
                    buf.clear();
                }
                chars.next();
            }
            _ => {
                buf.push(ch);
                chars.next();
            }
        }
    }
    if !buf.trim().is_empty() {
        tokens.push(buf.trim().to_string());
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize_with_redirect() {
        let line = "echo hello > file.txt".to_string();
        let tokens = get_tokens(line);
        assert_eq!(tokens, vec!["echo", "hello", ">", "file.txt"]);
    }
    #[test]
    fn test_tokenize_with_spaces_and_quotes() {
        let line = "echo \"a > b\" > out.txt".to_string();
        let tokens = get_tokens(line);
        assert_eq!(tokens, vec!["echo", "\"a > b\"", ">", "out.txt"]);
    }
    #[test]
    fn test_tokenize_multiple_spaces() {
        let line = "ls    -l    >     result.txt".to_string();
        let tokens = get_tokens(line);
        assert_eq!(tokens, vec!["ls", "-l", ">", "result.txt"]);
    }
    #[test]
    fn test_tokenize_redirect_last_wins() {
        let line = "echo foo > one.txt > two.txt".to_string();
        let tokens = get_tokens(line);
        assert_eq!(tokens, vec!["echo", "foo", ">", "one.txt", ">", "two.txt"]);
    }
}
