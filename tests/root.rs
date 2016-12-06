extern crate pureconfig;

use pureconfig::Config;

#[test]
fn root_level_three_properties() {
    let config: Config = "hostname = \"dynamo\"\nport = \"5153\"\npath = \"/foo/bar\""
        .parse()
        .unwrap();
    assert_eq!(config.get("path"), Some("/foo/bar"));
}

#[test]
#[ignore]
fn simple_multiline() {
    let config: Config = "host\n    name = dynamo".parse().unwrap();
    assert_eq!(config.get("host.name"), Some("dynamo"));
}
