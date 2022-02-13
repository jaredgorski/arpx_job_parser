use crate::generic::Parser;

pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::literal;

    #[test]
    fn test_either() -> Result<(), String> {
        let parser = either(literal("foo"), literal("bar"));

        assert_eq!(parser.parse("foo")?, ("", ()));
        assert_eq!(parser.parse("bar")?, ("", ()));
        assert_eq!(parser.parse("baz"), Err("baz"));
        Ok(())
    }
}
