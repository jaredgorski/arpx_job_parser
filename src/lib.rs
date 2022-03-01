pub mod arpx_job;
mod error;
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

pub use arpx_job::{Job, Process, Task};
pub use generic::combinators::parser::{ParseResult, Parser};

use error::{get_parse_error_context, ParseErrorContext};

pub fn parse_job(job: &str) -> Result<arpx_job::Job, ParseErrorContext> {
    match arpx_job::job().parse(job) {
        Ok(("", parsed_job)) => Ok(parsed_job),
        Ok((remaining, _)) => {
            let error_context = get_parse_error_context(job, remaining);

            Err(error_context)
        }
        Err(error) => panic!("{:?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_job, Job, ParseErrorContext, Process, Task};

    #[test]
    fn test_parse_job() -> Result<(), ParseErrorContext> {
        let example = r#"
            [
                loop1 ? loop2 : loop3; @monitor_1 @monitor_2
                loop2 ? loop3 : loop4;
            ]
            loop3 ? loop4 : loop5; @monitor_3
            loop6;
            loop7 ? loop8; @monitor_4
        "#;

        let expected = Job {
            tasks: vec![
                Task {
                    processes: vec![
                        Process {
                            log_monitors: vec!["monitor_1".to_string(), "monitor_2".to_string()],
                            name: "loop1".to_string(),
                            onsucceed: Some("loop2".to_string()),
                            onfail: Some("loop3".to_string()),
                        },
                        Process {
                            log_monitors: Vec::new(),
                            name: "loop2".to_string(),
                            onsucceed: Some("loop3".to_string()),
                            onfail: Some("loop4".to_string()),
                        },
                    ],
                },
                Task {
                    processes: vec![Process {
                        log_monitors: vec!["monitor_3".to_string()],
                        name: "loop3".to_string(),
                        onsucceed: Some("loop4".to_string()),
                        onfail: Some("loop5".to_string()),
                    }],
                },
                Task {
                    processes: vec![Process {
                        log_monitors: Vec::new(),
                        name: "loop6".to_string(),
                        onsucceed: None,
                        onfail: None,
                    }],
                },
                Task {
                    processes: vec![Process {
                        log_monitors: vec!["monitor_4".to_string()],
                        name: "loop7".to_string(),
                        onsucceed: Some("loop8".to_string()),
                        onfail: None,
                    }],
                },
            ],
        };

        assert_eq!(parse_job(example)?, expected);
        Ok(())
    }
}
