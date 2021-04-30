extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate schemars;

pub mod coffeebot;
mod types;
// list any additional modules here

pub use crate::types::Coffeebot;
pub use crate::types::State;