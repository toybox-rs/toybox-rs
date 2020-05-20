//! The breakout crate contains the data structures and logic for a clone of the Atari 2600 game Breakout, but defined to be more flexible.
//!
//! None of the modules in this crate are public. The `Breakout` struct is the `toybox_core::Simulation` and the `State` struct is the `toybox_core::State` used generically by other crates.

extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate schemars;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate rand;

/// Use 2d types shared with tb_pong.
pub use toybox_core::body2d::Body2D;
pub use toybox_core::vec2d::Vec2D;

/// This module contains the core logic of the game.
mod breakout;
/// This module contains the font used for rendering scores.
mod font;
/// This module contains the core data structures used in the game.
mod types;

pub use crate::types::{Breakout, Brick, StartBall, State, StateCore};
