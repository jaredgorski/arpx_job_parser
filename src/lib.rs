mod arpx_job;
mod generic;

mod prelude {
    pub use crate::generic::any_char::any_char;
    pub use crate::generic::either::either;
    pub use crate::generic::left::left;
    pub use crate::generic::literal::literal;
    pub use crate::generic::n::n;
    pub use crate::generic::optional::optional;
    pub use crate::generic::pair::pair;
    pub use crate::generic::parser::{ParseResult, Parser};
    pub use crate::generic::right::right;
    pub use crate::generic::whitespace::whitespace_wrap;
}

pub use crate::arpx_job::{job, task, Job, Process, Task};
