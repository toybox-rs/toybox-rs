extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate schemars;
extern crate rand;

pub mod amidar;
mod digit_sprites;
mod types;

pub use crate::types::Amidar;
pub use crate::types::State;
