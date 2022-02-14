use crate::generic::any_char::any_char;
use crate::generic::left::left;
use crate::generic::parser::Parser;
use crate::generic::right::right;
use crate::generic::zero_or_more::zero_or_more;

pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: 'a + Parser<'a, A>,
    A: 'a,
{
    right(space0(), left(parser, space0()))
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    any_char.pred(|c| c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::literal::literal;

    #[test]
    fn test_whitespace_wrap() -> Result<(), String> {
        let parser = whitespace_wrap(literal("foo"));

        assert_eq!(parser.parse("foo")?, ("", ()));
        assert_eq!(parser.parse("    foo     ")?, ("", ()));
        assert_eq!(
            parser.parse(
                r#"
                foo
            "#
            )?,
            ("", ())
        );
        assert_eq!(parser.parse("    foo        bar")?, ("bar", ()));
        assert_eq!(parser.parse("   bar   "), Err("bar   "));
        Ok(())
    }

    #[test]
    fn test_whitespace_char() -> Result<(), String> {
        let parser = whitespace_char();

        assert_eq!(parser.parse(" ")?, ("", ' '));
        assert_eq!(parser.parse("f"), Err("f"));
        assert_eq!(parser.parse(""), Err(""));
        Ok(())
    }
}
