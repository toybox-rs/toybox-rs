extern crate toybox_core;

use std::sync::{Arc, Mutex};

pub use toybox_core::graphics;
pub use toybox_core::random;
/// Input represents the buttons pressed given to our games.
pub use toybox_core::Input;
pub use toybox_core::SimFlag;
pub use toybox_core::Simulation;
pub use toybox_core::State;

use once_cell::sync::OnceCell;
static GAME_LIST: OnceCell<Vec<String>> = OnceCell::new();

fn game_list() -> &'static Vec<String> {
    GAME_LIST.get_or_init(|| {
        (inventory::iter::<SimFlag>())
            .map(|simflag| String::from(String::as_str(&simflag.name)))
            .collect()
    })
}

/// This method returns a Box<Simulation> if possible for a given game name.
pub fn get_simulation_by_name(name: &str) -> Result<Arc<Mutex<dyn Simulation>>, String> {
    for simflag in inventory::iter::<SimFlag>() {
        if simflag.name == name {
            return Ok(simflag.simulation.clone());
        }
    }
    Err(format!(
        "Cannot construct game: `{}`. Try any of {:?}.",
        name,
        game_list()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakout() {
        let game = get_simulation_by_name("breakout");
        assert!(game.is_ok())
    }
    #[test]
    fn test_games_list() {
        let games = game_list();
        assert!(games.len() > 0);
    }
}
