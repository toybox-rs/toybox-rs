#![crate_type = "dylib"]

extern crate amidar;
extern crate breakout;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate toybox;
extern crate toybox_core;

/// This struct represents a Simulator that hides rust's "fat" pointer implementation.
/// This struct is therefore whole as a single c void pointer, but the internals still have a pointer to both the trait and the actual impl.
pub struct WrapSimulator {
    pub simulator: Box<dyn toybox_core::Simulation>,
}

/// This struct represents a State that hides rust's "fat" pointer implementation.
/// This struct is therefore whole as a single c void pointer, but the internals still have a pointer to both the trait and the actual impl.
pub struct WrapState {
    pub state: Box<dyn toybox_core::State>,
}

mod core;
pub use core::*;
