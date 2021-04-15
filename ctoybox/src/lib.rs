#![crate_type = "dylib"]

extern crate amidar;
extern crate breakout;
extern crate libc;
extern crate pong;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toybox;
extern crate toybox_core;

mod ffi_result;
use ffi_result::FFIResult;
use std::sync::{Arc, Mutex};

/// This struct represents a Simulator that hides rust's "fat" pointer implementation.
/// This struct is therefore whole as a single c void pointer, but the internals still have a pointer to both the trait and the actual impl.
#[repr(C)]
pub struct WrapSimulator {
    pub simulator: Arc<Mutex<dyn toybox_core::Simulation>>,
}

/// This struct represents a State that hides rust's "fat" pointer implementation.
/// This struct is therefore whole as a single c void pointer, but the internals still have a pointer to both the trait and the actual impl.
#[repr(C)]
pub struct WrapState {
    pub state: Box<dyn toybox_core::State>,
}

/// Note: not-recursive. Free Error Message Manually!
#[no_mangle]
pub extern "C" fn free_ffi_result(originally_from_rust: *mut FFIResult) {
    let _will_drop: Box<FFIResult> = unsafe { Box::from_raw(originally_from_rust) };
}

mod core;
pub use crate::core::*;
