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

    fn from_lines(lines: Vec<Line>) -> Config {
        let mut map = HashMap::new();
        for line in lines {
            match line {
                Line::KeyValue(key, value) => {
                    map.insert(key, value);
                }
                Line::Comment(_) => {}
            };
        }
        Config { map: map }
    }
}

impl FromStr for Config {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = try!(get_lines(s));
        Ok(Config::from_lines(lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::parser::Line;

    #[test]
    fn single_key_value() {
        let config = Config::from_lines(vec![Line::KeyValue("hostname".to_string(),
                                                            "dynamo".to_string())]);
        assert_eq!(config.get("hostname"), Some("dynamo"));
    }

    #[test]
    fn missing_key_value() {
        let config = Config::from_lines(vec![Line::KeyValue("hostname".to_string(),
                                                            "dynamo".to_string())]);
        assert_eq!(config.get("port"), None);
    }

    #[test]
    fn many_key_values() {
        let config =
            Config::from_lines(vec![Line::KeyValue("hostname".to_string(), "dynamo".to_string()),
                                    Line::KeyValue("port".to_string(), "5153".to_string()),
                                    Line::KeyValue("path".to_string(), "/foo/bar".to_string())]);
        assert_eq!(config.get("hostname"), Some("dynamo"));
        assert_eq!(config.get("port"), Some("5153"));
        assert_eq!(config.get("path"), Some("/foo/bar"));
    }

    #[test]
    fn comments() {
        let config = Config::from_lines(vec![Line::Comment("anything".to_string())]);
        assert_eq!(config.get("hostname"), None);
    }
}
