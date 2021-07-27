extern crate toybox_core;

pub use toybox_core::graphics;
pub use toybox_core::random;
/// Input represents the buttons pressed given to our games.
pub use toybox_core::Input;
pub use toybox_core::Simulation;
pub use toybox_core::State;

/// This method returns a Box<Simulation> if possible for a given game name.
pub fn get_simulation_by_name(name: &str) -> Result<Box<dyn Simulation>, String> {
    match name.to_lowercase().as_str() {
        #[cfg(feature = "amidar")]
        "amidar" => Ok(Box::new(amidar::Amidar::default())),
        #[cfg(feature = "breakout")]
        "breakout" => Ok(Box::new(breakout::Breakout::default())),
        #[cfg(feature = "gridworld")]
        "gridworld" => Ok(Box::new(gridworld::GridWorld::default())),
        #[cfg(feature = "spaceinvaders")]
        "spaceinvaders" => Ok(Box::new(spaceinvaders::SpaceInvaders::default())),
        #[cfg(feature = "pong")]
        "pong" => Ok(Box::new(pong::Pong::default())),
        #[cfg(feature = "pitfall")]
        "pitfall" => Ok(Box::new(pitfall::Pitfall::default())),
        _ => Err(format!(
            "Cannot construct game: `{}`. Try any of {:?}.",
            name, GAME_LIST
        )),
    }
}

/// This defines the set of games that are known. An index into this array is used in human_play, so try not to shuffle them!
pub const GAME_LIST: &[&str] = &[
    #[cfg(feature = "amidar")]
    "amidar",
    #[cfg(feature = "breakout")]
    "breakout",
    #[cfg(feature = "gridworld")]
    "gridworld",
    #[cfg(feature = "spaceinvaders")]
    "spaceinvaders",
    #[cfg(feature = "pong")]
    "pong",
    #[cfg(feature = "pitfall")]
    "pitfall",
];

/// Amidar defined in this module.
#[cfg(feature = "amidar")]
extern crate amidar;

/// Breakout defined in this module.
#[cfg(feature = "breakout")]
extern crate breakout;

/// GridWorld defined in this module.
#[cfg(feature = "gridworld")]
extern crate gridworld;

/// SpaceInvaders defined in this module.
#[cfg(feature = "spaceinvaders")]
extern crate spaceinvaders;

/// Pong defined in this module.
#[cfg(feature = "pong")]
extern crate pong;

/// Pitfall defined in this module.
#[cfg(feature = "pitfall")]
extern crate pitfall;
