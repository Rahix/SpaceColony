// A map manager for SpaceColony
//
// (c) 2015 Rahix

use tomlreader::TomlReader;
use rustc_serialize::Decodable;

pub struct SpaceColonyMap {
    data: TomlReader,
}

impl SpaceColonyMap {
    pub fn new(save: String) -> SpaceColonyMap {
        let mut filepath = "data/saves/".to_string();
        filepath.push_str(&save);
        filepath.push_str(".toml");
        let data = TomlReader::new(&filepath);
        SpaceColonyMap {
            data: data,
        }
    }

    pub fn getnumplanets(&mut self) -> i32 {
        let num = self.data.get_value::<i32>("planets.num").expect("Save file corrupted!");
        num
    }

    pub fn getdata<T: Decodable>(&mut self, planet_id: i32, what: &str) -> Option<T> {
        let mut data_path = "planet.".to_string();
        data_path.push_str(&(planet_id - 1).to_string());
        data_path.push_str(".");
        data_path.push_str(what);
        self.data.get_value(&data_path)
    }
}
