extern crate pureconfig;

use pureconfig::Config;
use pureconfig::ParseError;

#[test]
fn root_level_empty() {
    let config: Config = "".parse().unwrap();
    assert_eq!(config.get("hostname"), None);
}

#[test]
fn root_level_single_property() {
    let config: Config = "hostname = \"dynamo\"".parse().unwrap();
    assert_eq!(config.get("hostname"), Some("dynamo"));
}

#[test]
fn root_level_three_properties() {
    let config: Config = "hostname = \"dynamo\"\nport = \"5153\"\npath = \"/foo/bar\""
        .parse()
        .unwrap();
    assert_eq!(config.get("path"), Some("/foo/bar"));
}

#[test]
fn just_a_word() {
    assert_eq!("hostname".parse::<Config>().unwrap_err(),
               ParseError::Syntax);
}

#[test]
fn word_and_equals() {
    assert_eq!("hostname = ".parse::<Config>().unwrap_err(),
               ParseError::Syntax);
}
