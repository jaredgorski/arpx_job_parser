pub mod arpx_job;
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

pub fn parse_job(job: &str) -> ParseResult<arpx_job::Job> {
    arpx_job::job().parse(job)
}

#[cfg(test)]
mod tests {
    use super::{parse_job, Job, Process, Task};

    #[test]
    fn test_parse_job() -> Result<(), String> {
        let example = r#"
            [
                (loop1 ? loop2 : loop3;)
                loop2 ? loop3 : loop4;
            ]
            loop3 ? loop4 : loop5;
            loop6;
            (loop7 ? loop8;)
        "#;

        let expected = (
            "",
            Job {
                tasks: vec![
                    Task {
                        processes: vec![
                            Process {
                                name: "loop1".to_string(),
                                onsucceed: Some(Box::new(Process {
                                    name: "loop2".to_string(),
                                    onsucceed: None,
                                    onfail: None,
                                    silent: false,
                                })),
                                onfail: Some(Box::new(Process {
                                    name: "loop3".to_string(),
                                    onsucceed: None,
                                    onfail: None,
                                    silent: false,
                                })),
                                silent: true,
                            },
                            Process {
                                name: "loop2".to_string(),
                                onsucceed: Some(Box::new(Process {
                                    name: "loop3".to_string(),
                                    onsucceed: None,
                                    onfail: None,
                                    silent: false,
                                })),
                                onfail: Some(Box::new(Process {
                                    name: "loop4".to_string(),
                                    onsucceed: None,
                                    onfail: None,
                                    silent: false,
                                })),
                                silent: false,
                            },
                        ],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop3".to_string(),
                            onsucceed: Some(Box::new(Process {
                                name: "loop4".to_string(),
                                onsucceed: None,
                                onfail: None,
                                silent: false,
                            })),
                            onfail: Some(Box::new(Process {
                                name: "loop5".to_string(),
                                onsucceed: None,
                                onfail: None,
                                silent: false,
                            })),
                            silent: false,
                        }],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop6".to_string(),
                            onsucceed: None,
                            onfail: None,
                            silent: false,
                        }],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop7".to_string(),
                            onsucceed: Some(Box::new(Process {
                                name: "loop8".to_string(),
                                onsucceed: None,
                                onfail: None,
                                silent: false,
                            })),
                            onfail: None,
                            silent: true,
                        }],
                    },
                ],
            },
        );

        assert_eq!(parse_job(example)?, expected);
        Ok(())
    }
}
