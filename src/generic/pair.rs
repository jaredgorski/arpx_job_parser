use crate::generic::parser::Parser;

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::any_char::any_char;
    use crate::generic::literal::literal;
    use crate::generic::one_or_more::one_or_more;

    #[test]
    fn test_pair() -> Result<(), String> {
        let alphanumeric = any_char.pred(|c| c.is_alphanumeric());
        let any_word =
            one_or_more(alphanumeric).map(|result| result.into_iter().collect::<String>());

        let parser = pair(literal("foo"), any_word);

        assert_eq!(parser.parse("foobar")?, ("", ((), "bar".to_string())));
        assert_eq!(parser.parse("baz"), Err("baz"));
        Ok(())
    }
}
