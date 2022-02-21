use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub name: String,
    pub onfail: String,
    pub onsucceed: String,
    pub silent: bool,
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
    whitespace_wrap(either(silent_process(), nonsilent_process()))
}

fn silent_process<'a>() -> impl Parser<'a, Vec<Process>> {
    right(
        whitespace_wrap(literal("(")),
        left(nonsilent_process(), whitespace_wrap(literal(")"))),
    )
    .map(|process_vec| match process_vec.first() {
        Some(process) => vec![Process {
            name: process.name.clone(),
            onfail: process.onfail.clone(),
            onsucceed: process.onsucceed.clone(),
            silent: true,
        }],
        None => panic!(),
    })
}

fn nonsilent_process<'a>() -> impl Parser<'a, Vec<Process>> {
    terminating_semicolon(pair(process_name, process_predicate())).map(
        |(name, (onsucceed, onfail))| {
            vec![Process {
                name,
                onfail,
                onsucceed,
                silent: false,
            }]
        },
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

fn process_predicate<'a>() -> impl Parser<'a, (String, String)> {
    pair(
        optional(onsucceed()).map(|result| match result {
            Some(input) => input,
            None => String::new(),
        }),
        optional(onfail()).map(|result| match result {
            Some(input) => input,
            None => String::new(),
        }),
    )
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
            name: "loop1".to_string(),
            onsucceed: String::new(),
            onfail: String::new(),
            silent: false,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_silent_process() -> Result<(), String> {
        let example = "(loop1;)";

        let expected = vec![Process {
            name: "loop1".to_string(),
            onsucceed: String::new(),
            onfail: String::new(),
            silent: true,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_only_onsucceed() -> Result<(), String> {
        let example = "loop1 ? loop2;";

        let expected = vec![Process {
            name: "loop1".to_string(),
            onsucceed: "loop2".to_string(),
            onfail: String::new(),
            silent: false,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_only_onfail() -> Result<(), String> {
        let example = "loop1 : loop3;";

        let expected = vec![Process {
            name: "loop1".to_string(),
            onsucceed: String::new(),
            onfail: "loop3".to_string(),
            silent: false,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_process_with_onsucceed_and_onfail() -> Result<(), String> {
        let example = "loop1 ? loop2 : loop3;";

        let expected = vec![Process {
            name: "loop1".to_string(),
            onsucceed: "loop2".to_string(),
            onfail: "loop3".to_string(),
            silent: false,
        }];

        assert_eq!(single_process().parse(example)?, ("", expected));
        Ok(())
    }

    #[test]
    fn test_silent_process_with_onsucceed_and_onfail() -> Result<(), String> {
        let example = "(loop1 ? loop2 : loop3;)";

        let expected = vec![Process {
            name: "loop1".to_string(),
            onsucceed: "loop2".to_string(),
            onfail: "loop3".to_string(),
            silent: true,
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
                (loop3 ? loop4 : loop5;)
            ]
        "#;

        let expected = vec![
            Process {
                name: "loop1".to_string(),
                onsucceed: "loop2".to_string(),
                onfail: "loop3".to_string(),
                silent: false,
            },
            Process {
                name: "loop2".to_string(),
                onsucceed: "loop3".to_string(),
                onfail: "loop4".to_string(),
                silent: false,
            },
            Process {
                name: "loop3".to_string(),
                onsucceed: "loop4".to_string(),
                onfail: "loop5".to_string(),
                silent: true,
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
                name: "loop1".to_string(),
                onsucceed: "loop2".to_string(),
                onfail: "loop3".to_string(),
                silent: false,
            },
            Process {
                name: "loop2".to_string(),
                onsucceed: "loop3".to_string(),
                onfail: String::new(),
                silent: false,
            },
        ];

        assert_eq!(concurrent_processes().parse(example_1), Err("]"));
        assert_eq!(concurrent_processes().parse(example_2)?, ("", expected_2));
        Ok(())
    }
}
