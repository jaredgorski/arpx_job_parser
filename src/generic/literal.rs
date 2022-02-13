use crate::generic::Parser;

pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() -> Result<(), String> {
        let parser = literal("foo");

        assert_eq!(parser.parse("foo")?, ("", ()));
        assert_eq!(parser.parse("foobar")?, ("bar", ()));
        assert_eq!(parser.parse("bar"), Err("bar"));
        Ok(())
    }
}
