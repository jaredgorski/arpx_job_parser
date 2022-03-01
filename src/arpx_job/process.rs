use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub log_monitors: Vec<String>,
    pub name: String,
    pub onfail: Option<String>,
    pub onsucceed: Option<String>,
}

pub fn concurrent_processes<'a>() -> impl Parser<'a, Vec<Process>> {
    right(
        whitespace_wrap(literal("[")),
        left(
            n(single_process(), 2..).map(|result| result.into_iter().flatten().collect()),
            whitespace_wrap(literal("]")),
        ),
    )
}

pub fn single_process<'a>() -> impl Parser<'a, Vec<Process>> {
    pair(process_without_log_monitors(), log_monitors()).map(|(process_vec, log_monitors)| {
        match process_vec.first() {
            Some(process) => vec![Process {
                log_monitors,
                name: process.name.clone(),
                onfail: process.onfail.clone(),
                onsucceed: process.onsucceed.clone(),
            }],
            None => panic!(),
        }
    })
}

pub fn log_monitors<'a>() -> impl Parser<'a, Vec<String>> {
    n(whitespace_wrap(right(literal("@"), process_name)), 0..)
}

fn process_without_log_monitors<'a>() -> impl Parser<'a, Vec<Process>> {
    whitespace_wrap(
        terminating_semicolon(pair(process_name, process_predicate())).map(
            |(name, (onsucceed, onfail))| {
                vec![Process {
                    log_monitors: Vec::new(),
                    name,
                    onfail,
                    onsucceed,
                }]
            },
        ),
    )
}

fn terminating_semicolon<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: 'a + Parser<'a, A>,
    A: 'a,
{
    left(parser, literal(";"))
}

fn process_name(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let chars = input.chars();

    for next in chars {
        if next.is_alphanumeric() || next == '-' || next == '_' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();

    Ok((&input[next_index..], matched))
}

fn process_predicate<'a>() -> impl Parser<'a, (Option<String>, Option<String>)> {
    pair(optional(onsucceed()), optional(onfail()))
}

fn onsucceed<'a>() -> impl Parser<'a, String> {
    right(whitespace_wrap(literal("?")), process_name)
}

fn onfail<'a>() -> impl Parser<'a, String> {
    right(whitespace_wrap(literal(":")), process_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arpx_job::process::Process;

    #[test]
    fn test_process() -> Result<(), String> {
        let example = "loop1;";

        let expected = vec![Process {
            log_monitors: Vec::new(),
            name: "loop1".to_string(),
            onsucceed: None,
            onfail: None,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_only_onsucceed() -> Result<(), String> {
        let example = "loop1 ? loop2;";

        let expected = vec![Process {
            log_monitors: Vec::new(),
            name: "loop1".to_string(),
            onsucceed: Some("loop2".to_string()),
            onfail: None,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_only_onfail() -> Result<(), String> {
        let example = "loop1 : loop3;";

        let expected = vec![Process {
            log_monitors: Vec::new(),
            name: "loop1".to_string(),
            onsucceed: None,
            onfail: Some("loop3".to_string()),
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_onsucceed_and_onfail() -> Result<(), String> {
        let example = "loop1 ? loop2 : loop3;";

        let expected = vec![Process {
            log_monitors: Vec::new(),
            name: "loop1".to_string(),
            onsucceed: Some("loop2".to_string()),
            onfail: Some("loop3".to_string()),
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_concurrent_processes() -> Result<(), String> {
        let example = r#"
            [
                loop1 ? loop2 : loop3;
                loop2 ? loop3 : loop4;
                loop3 ? loop4 : loop5;
            ]
        "#;

        let expected = vec![
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
            Process {
                log_monitors: Vec::new(),
                name: "loop3".to_string(),
                onsucceed: Some("loop4".to_string()),
                onfail: Some("loop5".to_string()),
            },
        ];

        assert_eq!(concurrent_processes().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_concurrent_processes_must_have_2_single_processes() -> Result<(), String> {
        let example_1 = "[loop1 ? loop2 : loop3;]";
        let example_2 = "[loop1 ? loop2 : loop3; loop2 ? loop3;]";

        let expected_2 = vec![
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
                onfail: None,
            },
        ];

        assert_eq!(concurrent_processes().parse(example_1), Err("]"));
        assert_eq!(concurrent_processes().parse(example_2)?, ("", expected_2));
        Ok(())
    }

    #[test]
    fn test_process_with_log_monitors() -> Result<(), String> {
        let example = "loop1; @foo @bar @baz";

        let expected = vec![Process {
            log_monitors: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            name: "loop1".to_string(),
            onsucceed: None,
            onfail: None,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_concurrent_processes_with_log_monitors() -> Result<(), String> {
        let example = r#"
            [
                loop1 ? loop2 : loop3; @foo @bar
                loop2 ? loop3 : loop4;
                loop3 ? loop4 : loop5; @baz
            ]
        "#;

        let expected = vec![
            Process {
                log_monitors: vec!["foo".to_string(), "bar".to_string()],
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
            Process {
                log_monitors: vec!["baz".to_string()],
                name: "loop3".to_string(),
                onsucceed: Some("loop4".to_string()),
                onfail: Some("loop5".to_string()),
            },
        ];

        assert_eq!(concurrent_processes().parse(example)?, ("", expected));
        Ok(())
    }
}
