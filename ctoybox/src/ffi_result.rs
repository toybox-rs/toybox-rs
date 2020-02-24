use libc::{c_char, c_void};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::ptr;

/// This is a JSON-API, not a C-API, really.
#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    error: String,
    context: String,
}

#[repr(C)]
pub struct FFIResult {
    pub error_message: *const c_void,
    pub success: *const c_void,
}

impl Default for FFIResult {
    fn default() -> Self {
        FFIResult {
            error_message: ptr::null(),
            success: ptr::null(),
        }
    }
}

/// Accept a string parameter!
pub(crate) fn accept_str(name: &str, input: *const c_void) -> Result<&str, Box<dyn Error>> {
    if input.is_null() {
        Err(format!("NULL pointer: {}", name))?;
    }
    let input: &CStr = unsafe { CStr::from_ptr(input as *const c_char) };
    Ok(input
        .to_str()
        .map_err(|_| format!("Could not parse {} pointer as UTF-8 string!", name))?)
}

/// Internal helper: convert string reference to pointer to be passed to Python/C. Heap allocation.
pub(crate) fn return_string(output: &str) -> *const c_void {
    let c_output: CString = CString::new(output).expect("Conversion to CString should succeed!");
    CString::into_raw(c_output) as *const c_void
}

pub(crate) fn empty_result_to_ffi(rust_result: Result<(), Box<dyn Error>>) -> *const FFIResult {
    let mut c_result = Box::new(FFIResult::default());
    match rust_result {
        Ok(_) => {}
        Err(e) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                error: "error".to_string(),
                context: format!("{:?}", e),
            })
            .unwrap();
            c_result.error_message = return_string(&error_message);
        }
    };
    Box::into_raw(c_result)
}

pub(crate) fn str_result_to_ffi(rust_result: Result<String, Box<dyn Error>>) -> *const FFIResult {
    let mut c_result = Box::new(FFIResult::default());
    match rust_result.and_then(|rust_str| Ok(CString::new(rust_str)?)) {
        Ok(item) => {
            c_result.success = CString::into_raw(item) as *const c_void;
        }
        Err(e) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                error: "error".to_string(),
                context: format!("{:?}", e),
            })
            .unwrap();
            c_result.error_message = return_string(&error_message);
        }
    };
    Box::into_raw(c_result)
}

pub(crate) fn result_to_ffi<T>(rust_result: Result<T, Box<dyn Error>>) -> *const FFIResult {
    let mut c_result = Box::new(FFIResult::default());
    match rust_result {
        Ok(item) => {
            let output = Box::new(item);
            c_result.success = Box::into_raw(output) as *const c_void;
        }
        Err(e) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                error: "error".to_string(),
                context: format!("{:?}", e),
            })
            .unwrap();
            c_result.error_message = return_string(&error_message);
        }
    };
    Box::into_raw(c_result)
}
