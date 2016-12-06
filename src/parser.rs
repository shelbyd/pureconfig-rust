use std::str::from_utf8;
use nom::{not_line_ending, GetInput};

named!(quoted,
       delimited!(tag!("\""), take_until!("\""), tag!("\"")));

named!(property, take_until!(" = "));

named!(key_value<&[u8], Line>,
       do_parse!(key: property >>
                 tag!(" = ") >>
                 value: alt!(
                     quoted |
                     not_line_ending
                 ) >>
                 (KeyValue(to_string(key), to_string(value)))));

named!(line<&[u8], Line>, ws!(key_value));

named!(parse<&[u8], Vec<Line> >, many0!(line));

#[derive(Debug)]
pub enum Line {
    KeyValue(String, String),
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
