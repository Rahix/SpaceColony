// The main file containing the entry point of the program
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate toml;
extern crate rustc_serialize;

mod tomlreader;
mod gamestate;
mod map;

use tomlreader::TomlReader;
use gamestate::gamestate::{ChangeAction, states, GameState};
use map::SpaceColonyMap;


fn action_1() {
    println!("Action 1 called");
}

fn action_2() {
    println!("Action2 called");
}

fn main() {
    // Some code
    let mut props = TomlReader::new("properties.toml");
    println!("Loading Properies . . .");
    println!("Check: ResInfo = {}", props.get_value::<String>("Resources.ResInfo").unwrap());
    let mut gs = GameState::new();
    let change1 = ChangeAction::new(states::INITIALIZING, action_1);
    let change2 = ChangeAction::new_with_last(states::MAIN_MENU, states::INITIALIZING, action_2);
    gs.register_change_action(change1);
    gs.register_change_action(change2);
    println!("Changing state");
    gs.trigger_state_change(states::INITIALIZING);
    gs.trigger_state_change(states::MAIN_MENU);
    let mut map = SpaceColonyMap::new("tmp".to_string());
    println!("Numplanets: {}", map.getnumplanets().to_string());
    let data = map.getdata::<i32>(1, "orbit_radius").unwrap();
    println!("Orbit[1]: {}", data.to_string());
}
