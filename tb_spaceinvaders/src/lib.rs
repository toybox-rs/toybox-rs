extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate schemars;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod destruction;
mod firing_ai;
mod font;
mod spaceinvaders;
mod types;

// All types are essentially "public" API.
pub use crate::firing_ai::FiringAI;
pub use crate::types::Enemy;
pub use crate::types::Laser;
pub use crate::types::Player;
pub use crate::types::SpaceInvaders;
pub use crate::types::State;
pub use crate::types::StateCore;
pub use crate::types::Ufo;
