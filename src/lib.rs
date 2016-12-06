use std::str::FromStr;

#[macro_use]
extern crate nom;

use std::collections::HashMap;

mod parser;
use parser::{get_lines, Line};
pub use parser::ParseError;

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
                Line::KeyValue(key, value) => map.insert(key, value),
            };
        }
        Ok(Config { map: map })
    }
}
