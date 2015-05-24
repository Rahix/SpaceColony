// Configuration loader for SpaceColony
//
// (c) 2015 Rahix
use std::fs::File;
use std::io::Read;
use std::path::Path;
use rustc_serialize::Decodable;

use toml::{self, Value};

pub struct TomlReader {
    toml_map: Value,
}

impl TomlReader {
    pub fn new(filename: &str) ->TomlReader {
        // Load properties.toml
        // And parse
        let mut toml_file = File::open(Path::new(filename)).ok().expect("File broken");
        let mut file_content = String::new();
        toml_file.read_to_string(&mut file_content).unwrap(); //Maybe .unwrap();

        // Create Parser
        let mut toml_parser = toml::Parser::new(file_content.as_ref());

        let table = toml_parser.parse().unwrap();
        TomlReader { toml_map: Value::Table(table) }
    }

    pub fn get_value<T: Decodable>(&mut self, key: &str) -> Option<T> {
        match self.toml_map.lookup(key) {
            Some(value) => toml::decode::<T>(value.clone()),
            None => None,
        }
    }
}
