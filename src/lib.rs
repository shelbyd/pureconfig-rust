use std::str::{FromStr, from_utf8};

#[macro_use]
extern crate nom;

use nom::{alpha, not_line_ending, GetInput};
use std::collections::HashMap;

named!(quoted,
       delimited!(tag!("\""), take_until!("\""), tag!("\"")));

named!(property,
       recognize!(many1!(chain!(
                    alpha ~
                    opt!(tag!(".")),
                    || {}
                ))));

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

fn to_string(bytes: &[u8]) -> String {
    from_utf8(bytes)
        .expect("Managed to parse but failed to convert non-utf8")
        .to_string()
}

#[derive(Debug)]
enum Line {
    KeyValue(String, String),
}
use Line::*;

#[derive(Debug)]
pub struct Config {
    map: HashMap<String, String>,
}

impl Config {
    pub fn get(&self, property_name: &str) -> Option<&str> {
        self.map.get(property_name).map(|s| s.as_str())
    }
}

impl FromStr for Config {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = try!(get_lines(s));

        let mut map = HashMap::new();
        for line in lines {
            match line {
                KeyValue(key, value) => map.insert(key, value),
            };
        }
        Ok(Config { map: map })
    }
}

fn get_lines(s: &str) -> Result<Vec<Line>, ParseError> {
    let parsed = parse(s.as_bytes());
    match parsed.remaining_input() {
        Some(bytes) if (bytes.len() != 0) => return Err(Syntax),
        _ => {}
    }
    parsed.to_full_result().map_err(::std::convert::From::from)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Syntax,
}
use ParseError::*;

impl From<nom::IError> for ParseError {
    fn from(error: nom::IError) -> ParseError {
        match error {
            nom::IError::Incomplete(_) => Syntax,
            _ => panic!("Error evaluating parser"),
        }
    }
}
