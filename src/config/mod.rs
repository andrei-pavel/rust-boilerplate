use std::fs;

use clap::{Arg, App};
use yaml_rust::{YamlLoader, yaml::Hash};

pub fn configure() -> Hash {
    let matches = App::new("rust-boilerplate")
        .author("Andrei Pavel <andrei.pavel@cti.pub.ro>")
        .about("bootstrapping Rust boilerplate")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .required(true)
            .help("Provides the configuration file"))
        .get_matches();
    let config = matches.value_of("config").unwrap_or("./config.yaml");
    let contents = fs::read_to_string(config).expect("Unable to open file");
    let contents_slice: &str = &contents[..];
    let docs = YamlLoader::load_from_str(contents_slice).unwrap();
    return docs[0]["data"].as_hash().unwrap().clone();
}
