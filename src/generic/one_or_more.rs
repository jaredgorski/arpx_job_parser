use crate::generic::parser::Parser;

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_result)) = parser.parse(input) {
            input = next_input;
            result.push(first_result);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_result)) = parser.parse(input) {
            input = next_input;
            result.push(next_result);
        }

        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::literal::literal;

    #[test]
    fn test_one_or_more() -> Result<(), String> {
        let parser = one_or_more(literal("foo"));

        assert_eq!(parser.parse("foofoofoo")?, ("", vec![(), (), ()]));
        assert_eq!(parser.parse("bar"), Err("bar"));
        assert_eq!(parser.parse(""), Err(""));
        Ok(())
    }
}