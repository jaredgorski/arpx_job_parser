use std::iter::Iterator;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

use crate::generic::parser::Parser;

pub fn n<'a, P, R, A>(parser: P, range: R) -> impl Parser<'a, Vec<A>>
where
    R: Iterator<Item = usize> + RangeBounds<usize>,
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        let start = match range.start_bound() {
            Unbounded => 0,
            Included(value) => *value,
            Excluded(value) => value + 1,
        };
        let end = match range.end_bound() {
            // End bound must exist because this is a Range, not a RangeFrom.
            // If a larger range is needed, pass explicitly in range argument.
            Unbounded => 999999,
            Included(value) => value + 1,
            Excluded(value) => *value,
        };

        for num in 0..end {
            if let Ok((next_input, next_result)) = parser.parse(input) {
                input = next_input;
                result.push(next_result);
            } else if num < start {
                return Err(input);
            } else {
                break;
            }
        }

        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::literal::literal;

    #[test]
    fn test_n_or_more() -> Result<(), String> {
        let parser = n(literal("foo"), 3..);

        assert_eq!(parser.parse("foofoofoofoo")?, ("", vec![(), (), (), ()]));
        assert_eq!(parser.parse("foofoofoo")?, ("", vec![(), (), ()]));
        assert_eq!(parser.parse("foofoo"), Err(""));
        assert_eq!(parser.parse("bar"), Err("bar"));
        assert_eq!(parser.parse(""), Err(""));
        Ok(())
    }

    #[test]
    fn test_up_to_n() -> Result<(), String> {
        let parser = n(literal("foo"), 0..3);

        assert_eq!(parser.parse("foo")?, ("", vec![()]));
        assert_eq!(parser.parse("foofoo")?, ("", vec![(), ()]));
        assert_eq!(parser.parse("foofoofoo")?, ("", vec![(), (), ()]));
        assert_eq!(parser.parse("foofoofoofoo")?, ("foo", vec![(), (), ()]));
        assert_eq!(parser.parse("bar")?, ("bar", vec![]));
        assert_eq!(parser.parse("")?, ("", vec![]));
        Ok(())
    }

    #[test]
    fn test_up_to_and_including_n() -> Result<(), String> {
        let parser = n(literal("foo"), 0..=3);

        assert_eq!(parser.parse("foo")?, ("", vec![()]));
        assert_eq!(parser.parse("foofoo")?, ("", vec![(), ()]));
        assert_eq!(parser.parse("foofoofoo")?, ("", vec![(), (), ()]));
        assert_eq!(parser.parse("foofoofoofoo")?, ("", vec![(), (), (), ()]));
        assert_eq!(parser.parse("bar")?, ("bar", vec![]));
        assert_eq!(parser.parse("")?, ("", vec![]));
        Ok(())
    }
}
