use crate::generic::combinators::parser::ParseResult;

pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_char() -> Result<(), String> {
        let parse = any_char;

        assert_eq!(parse("foo")?, ("oo", 'f'));
        assert_eq!(parse(""), Err(""));
        Ok(())
    }
}
