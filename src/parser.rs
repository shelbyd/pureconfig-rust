use std::str::from_utf8;
use nom::{line_ending, not_line_ending, GetInput};

named!(quoted,
       delimited!(tag!("\""), take_until!("\""), tag!("\"")));

named!(quote_and_done,
       terminated!(quoted, alt!(eof!() | line_ending)));

named!(property, take_until!(" = "));

named!(bare_value, preceded!(not!(tag!("\"")), not_line_ending));

named!(key_value<&[u8], Line>,
       do_parse!(key: property >>
                 tag!(" = ") >>
                 value: alt!(
                     quote_and_done |
                     bare_value
                 ) >>
                 (KeyValue(to_string(key), to_string(value)))));

named!(comment<&[u8], Line>,
       do_parse!(
           content: preceded!(tag!("#"), not_line_ending) >>
           (Comment(to_string(content)))));

named!(line<&[u8], Line>, alt!(
        ws!(key_value) |
        ws!(comment)));

named!(parse<&[u8], Vec<Line> >, many0!(line));

#[derive(Debug, PartialEq)]
pub enum Line {
    KeyValue(String, String),
    Comment(String),
}
use self::Line::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Syntax,
}
use self::ParseError::*;

impl From<::nom::IError> for ParseError {
    fn from(error: ::nom::IError) -> ParseError {
        match error {
            ::nom::IError::Incomplete(_) => Syntax,
            _ => panic!("Error evaluating parser"),
        }
    }
}

fn to_string(bytes: &[u8]) -> String {
    from_utf8(bytes)
        .expect("Managed to parse but failed to convert non-utf8")
        .to_string()
}

pub fn get_lines(s: &str) -> Result<Vec<Line>, ParseError> {
    let parsed = parse(s.as_bytes());
    match parsed.remaining_input() {
        Some(bytes) if (bytes.len() != 0) => return Err(Syntax),
        _ => {}
    }
    parsed.to_full_result().map_err(::std::convert::From::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_syntax_error(contents: &str) {
        assert_eq!(get_lines(contents).unwrap_err(),
               ParseError::Syntax);
    }

    fn get_lines_vec(contents: &str) -> Vec<Line> {
        get_lines(contents).unwrap()
    }

    fn key_value(key: &str, value: &str) -> super::Line {
        super::Line::KeyValue(key.to_string(), value.to_string())
    }

    #[test]
    fn root_level_empty() {
        assert_eq!(get_lines_vec(""), vec![]);
    }

    #[test]
    fn root_level_single_property() {
        let result = vec![
            key_value("hostname", "dynamo"),
        ];
        assert_eq!(get_lines("hostname = \"dynamo\"").unwrap(), result);
    }

    #[test]
    fn root_level_three_properties() {
        let result = vec![
            key_value("hostname", "dynamo"),
            key_value("port", "5153"),
            key_value("path", "/foo/bar"),
        ];
        assert_eq!(
            get_lines("hostname = \"dynamo\"\nport = \"5153\"\npath = \"/foo/bar\"").unwrap(),
            result);
    }

    #[test]
    fn just_a_word() {
        assert_syntax_error("hostname");
    }

    #[test]
    fn word_and_equals() {
        assert_syntax_error("hostname = ");
    }

    #[test]
    #[ignore]
    fn root_level_property_with_many_dots_together() {
        assert_syntax_error("host..name = \"dynamo\"");
    }

    #[test]
    fn quoted_word_with_stuff_after() {
        assert_syntax_error("hostname = \"dynamo\"great");
        assert_syntax_error("hostname = \"dynamo\" great");
    }

    #[test]
    fn root_level_property_with_dots() {
        let result = vec![
            key_value("host.name", "dynamo"),
        ];
        assert_eq!(get_lines("host.name = \"dynamo\"").unwrap(), result);
    }

    #[test]
    fn root_level_property_with_underscores() {
        let result = vec![
            key_value("host_name", "dynamo"),
        ];
        assert_eq!(get_lines("host_name = \"dynamo\"").unwrap(), result);
    }

    #[test]
    fn bare_words() {
        let result = vec![
            key_value("hostname", "dynamo"),
        ];
        assert_eq!(get_lines("hostname = dynamo").unwrap(), result);
    }

    #[test]
    fn multiline_bare_words() {
        let result = vec![
            key_value("hostname", "dynamo"),
            key_value("port", "5153"),
            key_value("path", "/foo/bar"),
        ];
        assert_eq!(
            get_lines("hostname = dynamo\nport = 5153\npath = /foo/bar").unwrap(),
            result);
    }

    #[test]
    fn bare_words_until_end_of_line() {
        let result = vec![
            key_value("hostname", "dynamo is bae"),
        ];
        assert_eq!(get_lines("hostname = dynamo is bae").unwrap(), result);
    }

    #[test]
    fn comments() {
        let result = vec![
            Line::Comment(" Just a comment.".to_string()),
        ];
        assert_eq!(get_lines("# Just a comment.").unwrap(), result);
    }

    #[test]
    fn comment_after_bare_words() {
        let result = vec![
            key_value("hostname", "dynamo # This isn't actually a comment."),
        ];
        assert_eq!(
            get_lines("hostname = dynamo # This isn't actually a comment.").unwrap(),
            result);
    }

    #[test]
    fn comment_after_quote() {
        assert_syntax_error("hostname = \"dynamo\" # This isn't actually a comment.");
    }

    #[test]
    #[ignore]
    fn simple_multiline() {
        // let result = vec![
        //     Namespace("host".to_string(), vec![
        //         key_value("name", "dynamo"),
        //     ]),
        // ];
        // assert_eq!(get_lines("host\n    name = dynamo").unwrap(), result);
    }
}
