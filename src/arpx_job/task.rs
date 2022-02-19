use crate::arpx_job::process::{concurrent_processes, single_process, Process};
use crate::prelude::*;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub processes: Vec<Process>,
}

impl Deref for Task {
    type Target = Vec<Process>;

    fn deref(&self) -> &Self::Target {
        &self.processes
    }
}

#[must_use]
pub fn task<'a>() -> impl Parser<'a, Task> {
    either(concurrent_processes(), single_process()).map(|processes| Task { processes })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arpx_job::process::Process;

    #[test]
    fn test_single_task() -> Result<(), String> {
        let example = r#"
            loop1 ? loop2 : loop3;
        "#;

        let expected = Task {
            processes: vec![Process {
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
                silent: false,
            }],
        };

        assert_eq!(task().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_concurrent_task() -> Result<(), String> {
        let example = r#"
            [
                loop1 ? loop2 : loop3;
                loop2 ? loop3 : loop4;
            ]
        "#;

        let expected = Task {
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
                    silent: false,
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
        };

        assert_eq!(task().parse(example)?, ("", expected));
        Ok(())
    }
}
