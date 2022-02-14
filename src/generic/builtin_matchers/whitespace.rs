use crate::generic::builtin_matchers::any_char::any_char;
use crate::generic::combinators::left::left;
use crate::generic::combinators::n::n;
use crate::generic::combinators::parser::Parser;
use crate::generic::combinators::right::right;

pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: 'a + Parser<'a, A>,
    A: 'a,
{
    right(space0(), left(parser, space0()))
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    n(whitespace_char(), 0..)
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    any_char.pred(|c| c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::builtin_matchers::literal::literal;

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
