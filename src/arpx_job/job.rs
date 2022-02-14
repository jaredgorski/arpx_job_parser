use crate::arpx_job::task::{task, Task};
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Job {
    pub tasks: Vec<Task>,
}

#[must_use]
pub fn job<'a>() -> impl Parser<'a, Job> {
    n(whitespace_wrap(task()), 1..).map(|tasks| Job { tasks })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arpx_job::process::Process;

    #[test]
    fn test_job() -> Result<(), String> {
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
                                onsucceed: "loop2".to_string(),
                                onfail: "loop3".to_string(),
                                silent: true,
                            },
                            Process {
                                name: "loop2".to_string(),
                                onsucceed: "loop3".to_string(),
                                onfail: "loop4".to_string(),
                                silent: false,
                            },
                        ],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop3".to_string(),
                            onsucceed: "loop4".to_string(),
                            onfail: "loop5".to_string(),
                            silent: false,
                        }],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop6".to_string(),
                            onsucceed: String::new(),
                            onfail: String::new(),
                            silent: false,
                        }],
                    },
                    Task {
                        processes: vec![Process {
                            name: "loop7".to_string(),
                            onsucceed: "loop8".to_string(),
                            onfail: String::new(),
                            silent: true,
                        }],
                    },
                ],
            },
        );

        assert_eq!(job().parse(example)?, expected);
        Ok(())
    }
}
