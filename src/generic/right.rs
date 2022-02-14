use crate::generic::pair::pair;
use crate::generic::parser::Parser;

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: 'a + Parser<'a, R1>,
    P2: 'a + Parser<'a, R2>,
    R1: 'a,
    R2: 'a,
{
    pair(parser1, parser2).map(|(_left, right)| right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::any_char::any_char;
    use crate::generic::literal::literal;
    use crate::generic::one_or_more::one_or_more;

    #[test]
    fn test_right() -> Result<(), String> {
        let alphanumeric = any_char.pred(|c| c.is_alphanumeric());
        let any_word =
            one_or_more(alphanumeric).map(|result| result.into_iter().collect::<String>());

        let parser = right(literal("foo"), any_word);

        assert_eq!(parser.parse("foobar")?, ("", "bar".to_string()));
        assert_eq!(parser.parse("baz"), Err("baz"));
        Ok(())
    }
}
