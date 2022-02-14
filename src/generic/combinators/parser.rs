pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }

        Err(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::builtin_matchers::any_char::any_char;
    use crate::generic::builtin_matchers::literal::literal;
    use crate::generic::combinators::left::left;
    use crate::generic::combinators::n::n;

    #[test]
    fn test_map() -> Result<(), String> {
        let parser = map(any_char, |result| match result {
            'a' => "foo",
            _ => "bar",
        });

        assert_eq!(parser.parse("a")?, ("", "foo"));
        assert_eq!(parser.parse("z")?, ("", "bar"));
        Ok(())
    }

    #[test]
    fn test_and_then() -> Result<(), String> {
        let alphanumeric = any_char.pred(|c| c.is_alphanumeric());
        let any_word = n(alphanumeric, 1..).map(|result| result.into_iter().collect::<String>());
        let exclamation = left(any_word, literal("!"));

        let parser = and_then(exclamation, |result| match result.as_str() {
            "foo" => literal("bar"),
            _ => literal("baz"),
        });

        assert_eq!(parser.parse("foo!bar")?, ("", ()));
        assert_eq!(parser.parse("bar!baz")?, ("", ()));
        assert_eq!(parser.parse("foo!baz"), Err("baz"));
        assert_eq!(parser.parse("bar!bar"), Err("bar"));
        assert_eq!(parser.parse(""), Err(""));
        Ok(())
    }

    #[test]
    fn test_pred() -> Result<(), String> {
        let parser = pred(any_char, |c| *c == 'f');

        assert_eq!(parser.parse("foo")?, ("oo", 'f'));
        assert_eq!(parser.parse("bar"), Err("bar"));
        Ok(())
    }
}
