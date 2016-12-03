extern crate pureconfig;

use pureconfig::Config;

fn assert_syntax_error(contents: &str) {
    assert_eq!(contents.parse::<Config>().unwrap_err(),
               pureconfig::ParseError::Syntax);
}

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
    assert_syntax_error("hostname");
}

#[test]
fn word_and_equals() {
    assert_syntax_error("hostname = ");
}

#[test]
fn root_level_property_with_dots() {
    let config: Config = "host.name = \"dynamo\"".parse().unwrap();
    assert_eq!(config.get("host.name"), Some("dynamo"));
}

#[test]
fn root_level_property_with_many_dots_together() {
    assert_syntax_error("host..name = \"dynamo\"");
}

#[test]
fn bare_words() {
    let config: Config = "hostname = dynamo".parse().unwrap();
    assert_eq!(config.get("hostname"), Some("dynamo"));
}

#[test]
fn multiline_bare_words() {
    let config: Config = "hostname = dynamo\nport = 5153\npath = /foo/bar".parse().unwrap();
    assert_eq!(config.get("hostname"), Some("dynamo"));
    assert_eq!(config.get("path"), Some("/foo/bar"));
}

#[test]
fn bare_words_until_end_of_line() {
    let config: Config = "hostname = dynamo is bae".parse().unwrap();
    assert_eq!(config.get("hostname"), Some("dynamo is bae"));
}

#[test]
fn quoted_word_with_stuff_after() {
    assert_syntax_error("host..name = \"dynamo\"great");
    assert_syntax_error("host..name = \"dynamo\" great");
}
