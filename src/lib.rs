use std::str::{FromStr, from_utf8};

#[macro_use]
extern crate nom;

use nom::alpha;
use std::collections::HashMap;

named!(quoted,
       delimited!(tag!("\""), take_until!("\""), tag!("\"")));

named!(key_value<&[u8], (&[u8], &[u8])>,
       do_parse!(key: alpha >>
                 tag!(" = ") >>
                 value: quoted >>
                 (key, value)));

named!(line<&[u8], (&[u8], &[u8])>, ws!(key_value));

named!(parse<&[u8], Vec<(&[u8], &[u8])> >, many0!(line));

pub struct Config {
    map: HashMap<String, String>,
}

fn to_string(bytes: &[u8]) -> String {
    from_utf8(bytes).unwrap().to_string()
}

impl Config {
    pub fn get(&self, property_name: &str) -> Option<&str> {
        self.map.get(property_name).map(|s| s.as_str())
    }
}

impl FromStr for Config {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parse(s.as_bytes());
        let bytes = parsed.to_result().expect("Failed to parse.");
        let key_value_pairs = bytes.iter().map(|&(key, value)| (to_string(key), to_string(value)));
        let mut map = HashMap::new();
        for (key, value) in key_value_pairs {
            map.insert(key, value);
        }
        Ok(Config { map: map })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {}
