use std::cmp::{max, min};

type TextCoordinates = (u32, u32);

pub type ParseErrorContext = (TextCoordinates, String);

pub fn get_parse_error_context(source: &str, remaining: &str) -> ParseErrorContext {
    let char_pos = source.len() - remaining.len();

    let error_loc_slice = format!(
        " !ERROR-> {error_char}",
        error_char = source.as_bytes()[char_pos] as char
    );
    let before_pos = min(max(20, char_pos) - 20, char_pos);
    let after_pos = min(source.len() - 1, char_pos + 20);
    let context_before = &source[before_pos..char_pos];
    let context_after = &source[(char_pos + 1)..after_pos];
    let context = format!(
        "{before}{error}{after}",
        before = context_before,
        error = error_loc_slice,
        after = context_after
    );

    (get_text_coordinates(&source[..char_pos]), context)
}

fn get_text_coordinates(text: &str) -> TextCoordinates {
    let mut line = 1;
    let mut col = 0;

    for c in text.chars() {
        if c == '\n' {
            col = 0;
            line += 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

#[cfg(test)]
mod tests {
    use super::{get_parse_error_context, ParseErrorContext};

    #[test]
    fn test_parse_error_context() -> Result<(), ParseErrorContext> {
        let source = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Phasellus vulputate euismod luctus. Praesent viverra
dapibus nisl at feugiat. Fusce pellentesque augue ac nibh
tristique, nec tincidunt mauris lobortis. Vestibulum ante
ipsum primis in faucibus orci luctus et ultrices posuere
cubilia curae; Nam at ullamcorper ex. Nam pulvinar ligula
odio, eu pharetra arcu sagittis non. Phasellus ligula
massa, vestibulum eu diam vel, pharetra consequat enim.
Suspendisse potenti. Nullam consectetur ipsum sit amet eros
eleifend, id bibendum purus maximus. Praesent a libero sed
velit placerat placerat. Nam nec mi in magna vestibulum
dignissim quis ac lacus. Sed suscipit consequat auctor.
Proin commodo nibh a ultricies scelerisque. Pellentesque
pulvinar dui sed dolor interdum interdum. Vivamus dapibus
eros eu dolor faucibus interdum. Phasellus blandit sed
ipsum sed vestibulum."#;
        let remaining = &source[651..];

        let result = get_parse_error_context(source, remaining);

        let expected = (
            (12, 25),
            "ssim quis ac lacus.  !ERROR-> Sed suscipit consequ".to_string(),
        );

        assert_eq!(result, expected);
        Ok(())
    }
}
