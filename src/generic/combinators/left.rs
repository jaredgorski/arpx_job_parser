use crate::generic::combinators::pair::pair;
use crate::generic::combinators::parser::Parser;

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: 'a + Parser<'a, R1>,
    P2: 'a + Parser<'a, R2>,
    R1: 'a,
    R2: 'a,
{
    pair(parser1, parser2).map(|(left, _right)| left)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::builtin_matchers::any_char::any_char;
    use crate::generic::builtin_matchers::literal::literal;
    use crate::generic::combinators::n::n;

    #[test]
    fn test_left() -> Result<(), String> {
        let alphanumeric = any_char.pred(|c| c.is_alphanumeric());
        let any_word = n(alphanumeric, 1..).map(|result| result.into_iter().collect::<String>());

        let parser = left(any_word, literal("!"));

        assert_eq!(parser.parse("foo!")?, ("", "foo".to_string()));
        assert_eq!(parser.parse("baz"), Err(""));
        Ok(())
    }
}
