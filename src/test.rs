#[cfg(test)]

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn no_input() {
        let result = "$ ";
        assert_eq!(result, "")
    }
}
