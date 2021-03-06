use crate::arpx_job::task::{task, Task};
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Job {
    pub tasks: Vec<Task>,
}

#[must_use]
pub fn job<'a>() -> impl Parser<'a, Job> {
    n(whitespace_wrap(task()), 0..).map(|tasks| Job { tasks })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arpx_job::process::Process;

    #[test]
    fn test_job() -> Result<(), String> {
        let example = r#"
            [
                loop1 ? loop2 : loop3;
                loop2 ? loop3 : loop4;
            ]
            loop3 ? loop4 : loop5;
            loop6;
            loop7 ? loop8;
        "#;

        let expected = (
            "",
            Job {
                tasks: vec![
                    Task {
                        processes: vec![
                            Process {
                                log_monitors: Vec::new(),
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
                            log_monitors: Vec::new(),
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
                            log_monitors: Vec::new(),
                            name: "loop7".to_string(),
                            onsucceed: Some("loop8".to_string()),
                            onfail: None,
                        }],
                    },
                ],
            },
        );

        assert_eq!(job().parse(example)?, expected);
        Ok(())
    }
}
