use crate::ffi_result::*;
use crate::WrapSimulator;
use crate::WrapState;
use libc::{c_char, c_void};
use serde_json;
use std::boxed::Box;
use std::error::Error;
use std::ffi::CString;
use std::mem;
use toybox;
use toybox_core::graphics::{GrayscaleBuffer, ImageBuffer};
use toybox_core::{AleAction, Input};

fn get_simulator(ptr: *mut WrapSimulator) -> &'static mut dyn toybox::Simulation {
    assert!(!ptr.is_null());
    Box::leak(unsafe { Box::from_raw(ptr) }).simulator.as_mut()
}
fn get_state(ptr: *mut WrapState) -> &'static mut dyn toybox::State {
    assert!(!ptr.is_null());
    Box::leak(unsafe { Box::from_raw(ptr) }).state.as_mut()
}

/// Returns true if it received a non-null string to free.
#[no_mangle]
pub extern "C" fn free_str(originally_from_rust: *mut c_void) -> bool {
    if originally_from_rust.is_null() {
        return false;
    }
    let _will_drop: CString = unsafe { CString::from_raw(originally_from_rust as *mut c_char) };
    true
}

#[no_mangle]
pub extern "C" fn simulator_alloc(name: *const c_void) -> *const FFIResult {
    let sim = (|| {
        let name = accept_str("simulator_alloc(name)", name)?;
        let simulator = toybox::get_simulation_by_name(name)?;
        Ok(WrapSimulator { simulator })
    })();
    result_to_ffi(sim)
}

#[no_mangle]
pub extern "C" fn simulator_free(ptr: *mut WrapSimulator) -> bool {
    if ptr.is_null() {
        return false;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    true
}

// Reset the simulator RNG to a given seed.
#[no_mangle]
pub extern "C" fn simulator_seed(ptr: *mut WrapSimulator, seed: u32) {
    get_simulator(ptr).reset_seed(seed)
}

#[no_mangle]
pub extern "C" fn simulator_to_json(ptr: *mut WrapSimulator) -> *const c_void {
    let json = get_simulator(ptr).to_json();
    return_string(&json)
}

#[no_mangle]
pub extern "C" fn simulator_is_legal_action(ptr: *mut WrapSimulator, action: i32) -> bool {
    let actions = get_simulator(ptr).legal_action_set();
    if let Some(action) = AleAction::from_int(action) {
        actions.contains(&action)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn simulator_actions(ptr: *mut WrapSimulator) -> *const c_void {
    let actions: Vec<i32> = get_simulator(ptr)
        .legal_action_set()
        .into_iter()
        .map(|a| a.to_int())
        .collect();
    let actions = serde_json::to_string(&actions).expect("Vector to JSON should be OK.");
    return_string(&actions)
}

#[no_mangle]
pub extern "C" fn simulator_schema_for_state(ptr: *mut WrapSimulator) -> *const c_void {
    return_string(&get_simulator(ptr).schema_for_state())
}

#[no_mangle]
pub extern "C" fn simulator_schema_for_config(ptr: *mut WrapSimulator) -> *const c_void {
    return_string(&get_simulator(ptr).schema_for_config())
}

// STATE ALLOC + FREE
#[no_mangle]
pub extern "C" fn state_alloc(ptr: *mut WrapSimulator) -> *mut WrapState {
    let state = get_simulator(ptr).new_game();
    let boxed_wrapped_state = Box::new(WrapState { state });
    Box::into_raw(boxed_wrapped_state)
}

#[no_mangle]
pub extern "C" fn state_clone(ptr: *mut WrapState) -> *mut WrapState {
    let boxed_wrapped_state = Box::new(WrapState {
        state: get_state(ptr).copy(),
    });
    Box::into_raw(boxed_wrapped_state)
}

#[no_mangle]
pub extern "C" fn state_free(ptr: *mut WrapState) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

/// Hopefully the last "query" cbinding we need to write.
#[no_mangle]
pub extern "C" fn state_query_json(
    ptr: *mut WrapState,
    query_str: *const c_void,
    args_json_str: *const c_void,
) -> *const FFIResult {
    let response: Result<String, Box<dyn Error>> = (|| {
        let query_str = accept_str("query_str", query_str)?;
        let args_str = accept_str("args_json_str", args_json_str)?;
        let args: serde_json::Value = serde_json::from_str(args_str)?;
        Ok(get_state(ptr).query_json(query_str, &args)?)
    })();

    str_result_to_ffi(response)
}

// Need this information to initialize the numpy array in python
#[no_mangle]
pub extern "C" fn simulator_frame_width(ptr: *mut WrapSimulator) -> i32 {
    let (w, _) = get_simulator(ptr).game_size();
    w
}

#[no_mangle]
pub extern "C" fn simulator_frame_height(ptr: *mut WrapSimulator) -> i32 {
    let (_, h) = get_simulator(ptr).game_size();
    h
}

#[no_mangle]
pub extern "C" fn render_current_frame(
    numpy_pixels: *mut u8,
    numpy_pixels_len: usize,
    grayscale: bool,
    sim_ptr: *mut WrapSimulator,
    state_ptr: *mut WrapState,
) -> *const FFIResult {
    let rc: Result<(), Box<dyn Error>> = (|| {
        if numpy_pixels.is_null() {
            return Err("numpy_pixels is null".into());
        }
        let (w, h) = get_simulator(sim_ptr).game_size();
        let state = get_state(state_ptr);

        let imgdata = if grayscale {
            let mut img = GrayscaleBuffer::alloc(w, h);
            img.render(&state.draw());
            img.data
        } else {
            let mut img = ImageBuffer::alloc(w, h);
            img.render(&state.draw());
            img.data
        };

        if numpy_pixels_len != imgdata.len() {
            return Err(format!(
                "expected numpy_pixels_len={} to match rendered image: {}",
                numpy_pixels_len,
                imgdata.len()
            )
            .into());
        }

        let mut dat: Vec<u8> = unsafe { Vec::from_raw_parts(numpy_pixels, 0, numpy_pixels_len) };
        assert_eq!(dat.len(), 0);
        // Copy pixels at once (let LLVM/Rust optimize it as a linear copy).
        dat.extend(&imgdata);
        assert_eq!(dat.len(), imgdata.len());
        mem::forget(dat);
        Ok(())
    })();

    empty_result_to_ffi(rc)
}

#[no_mangle]
pub extern "C" fn state_apply_ale_action(state_ptr: *mut WrapState, input: i32) -> bool {
    if let Some(input) = AleAction::from_int(input).map(|a| a.to_input()) {
        get_state(state_ptr).update_mut(input);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn state_apply_action(
    state_ptr: *mut WrapState,
    input_ptr: *const c_void,
) -> *const FFIResult {
    let rc: Result<(), _> = (|| {
        let state = get_state(state_ptr);
        let input = accept_str("action_str", input_ptr)?;
        let input: Input = serde_json::from_str(input)?;
        state.update_mut(input);
        Ok(())
    })();

    empty_result_to_ffi(rc)
}

#[no_mangle]
pub extern "C" fn state_lives(state_ptr: *mut WrapState) -> i32 {
    get_state(state_ptr).lives()
}

#[no_mangle]
pub extern "C" fn state_level(state_ptr: *mut WrapState) -> i32 {
    get_state(state_ptr).level()
}

#[no_mangle]
pub extern "C" fn state_score(state_ptr: *mut WrapState) -> i32 {
    get_state(state_ptr).score()
}

#[no_mangle]
pub extern "C" fn state_handcrafted_features(state_ptr: *mut WrapState) -> *const c_void {
    return_string(
        &serde_json::to_string(&get_state(state_ptr).handcrafted_features())
            .expect("Map<String, f32> should be serializable to JSON!"),
    )
}

#[no_mangle]
pub extern "C" fn state_to_json(state_ptr: *mut WrapState) -> *const c_void {
    return_string(&get_state(state_ptr).to_json())
}

#[no_mangle]
pub extern "C" fn state_from_json(
    ptr: *mut WrapSimulator,
    json_str: *const c_void,
) -> *const FFIResult {
    let state: Result<WrapState, _> = (|| {
        let sim = get_simulator(ptr);
        let state = sim.new_state_from_json(accept_str("state_json_str", json_str)?)?;
        Ok(WrapState { state })
    })();
    result_to_ffi(state)
}

#[no_mangle]
pub extern "C" fn simulator_from_json(
    ptr: *mut WrapSimulator,
    json_str: *const c_void,
) -> *const FFIResult {
    let sim: Result<WrapSimulator, _> = (|| {
        let json_str = accept_str("config_json_str", json_str)?;
        let new_sim = get_simulator(ptr).from_json(json_str)?;
        Ok(WrapSimulator { simulator: new_sim })
    })();
    result_to_ffi(sim)
}
