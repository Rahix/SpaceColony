// Configuration loader for SpaceColony
//
// (c) 2015 Rahix
use std::fs::File;
use std::io::Read;
use std::path::Path;
use rustc_serialize::Decodable;

use toml::{self, Value};

pub struct PropertiesLoader {
    property_map: Value,
}

impl PropertiesLoader {
    pub fn new() -> PropertiesLoader {
        // Load properties.toml
        // And parse
        let mut property_file = File::open(Path::new("properties.toml")).ok().expect("Files broken");
        let mut file_content = String::new();
        property_file.read_to_string(&mut file_content).unwrap(); //Maybe .unwrap();

        // Create Parser
        let mut property_parser = toml::Parser::new(file_content.as_ref());

        let table = property_parser.parse().unwrap();
        PropertiesLoader { property_map: Value::Table(table) }
    }

    pub fn get_value<T: Decodable>(&mut self, key: &str) -> Option<T> {
        match self.property_map.lookup(key) {
            Some(value) => toml::decode::<T>(value.clone()),
            None => None,
        }
    }
}
