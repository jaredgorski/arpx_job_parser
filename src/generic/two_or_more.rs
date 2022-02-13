use crate::generic::Parser;

pub fn two_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
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

        if let Ok((next_input, second_result)) = parser.parse(input) {
            input = next_input;
            result.push(second_result);
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
    use crate::generic::literal;

    #[test]
    fn test_two_or_more() -> Result<(), String> {
        let parser = two_or_more(literal("foo"));

        assert_eq!(parser.parse("foofoofoo")?, ("", vec![(), (), ()]));
        assert_eq!(parser.parse("foofoo")?, ("", vec![(), ()]));
        assert_eq!(parser.parse("foo"), Err(""));
        assert_eq!(parser.parse("bar"), Err("bar"));
        assert_eq!(parser.parse(""), Err(""));
        Ok(())
    }
}
