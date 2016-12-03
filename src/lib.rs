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

named!(key_value<&[u8], (&[u8], &[u8])>,
       do_parse!(key: property >>
                 tag!(" = ") >>
                 value: alt!(
                     quoted |
                     not_line_ending
                 ) >>
                 (key, value)));

named!(line<&[u8], (&[u8], &[u8])>, ws!(key_value));

named!(parse<&[u8], Vec<(&[u8], &[u8])> >, many0!(line));

fn to_string(bytes: &[u8]) -> String {
    from_utf8(bytes)
        .expect("Managed to parse but failed to convert non-utf8")
        .to_string()
}

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
        let key_value_bytes = try!(get_byte_pairs(s));
        let key_value_pairs = key_value_bytes.iter()
            .map(|&(key, value)| (to_string(key), to_string(value)));

        let mut map = HashMap::new();
        for (key, value) in key_value_pairs {
            map.insert(key, value);
        }
        Ok(Config { map: map })
    }
}

fn get_byte_pairs(s: &str) -> Result<Vec<(&[u8], &[u8])>, ParseError> {
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
