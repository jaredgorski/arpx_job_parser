mod arpx_job;
mod generic;

mod prelude {
    pub use crate::generic::builtin_matchers::any_char::any_char;
    pub use crate::generic::builtin_matchers::literal::literal;
    pub use crate::generic::builtin_matchers::whitespace::whitespace_wrap;
    pub use crate::generic::combinators::either::either;
    pub use crate::generic::combinators::left::left;
    pub use crate::generic::combinators::n::n;
    pub use crate::generic::combinators::optional::optional;
    pub use crate::generic::combinators::pair::pair;
    pub use crate::generic::combinators::parser::{ParseResult, Parser};
    pub use crate::generic::combinators::right::right;
}

pub use arpx_job::{job, task, Job, Process, Task};
pub use generic::combinators::parser::{BoxedParser, ParseResult, Parser};

pub fn parse_job<'a>(job: &str) {
    job().parse(job)
}
