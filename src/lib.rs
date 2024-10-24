use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

mod contexts;
mod semantics;
mod mappings;
mod keycode_macos;
mod engine;

// Re-export the types we need
pub use crate::contexts::KeyState;
pub use crate::contexts::KeyEvent;
pub use crate::engine::PinkyTwirlEngine;

#[repr(C)]
pub struct FFIKeyEvent {
    pub key: *mut c_char,
    pub state: bool,  // true for down, false for up
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[no_mangle]
pub extern "C" fn pinkytwirl_engine_new(config_path: *const c_char) -> *mut PinkyTwirlEngine {
    let c_str = unsafe { CStr::from_ptr(config_path) };
    let path_str = c_str.to_str().unwrap_or("");
    let engine = PinkyTwirlEngine::new(PathBuf::from(path_str));
    Box::into_raw(Box::new(engine))
}

#[no_mangle]
pub extern "C" fn pinkytwirl_engine_free(ptr: *mut PinkyTwirlEngine) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr); }
    }
}

#[no_mangle]
pub extern "C" fn pinkytwirl_engine_handle_key_event(
    engine: *mut PinkyTwirlEngine,
    key_code: u16,
    down: bool,
    shift: bool,
    ctrl: bool,
    option: bool,
    meta: bool,
    app_name: *const c_char,
    window_name: *const c_char,
    out_length: *mut usize,
) -> *mut FFIKeyEvent {
    let engine = unsafe { &mut *engine };
    
    let app_name = unsafe { CStr::from_ptr(app_name) }.to_str().unwrap_or("");
    let window_name = unsafe { CStr::from_ptr(window_name) }.to_str().unwrap_or("");
    
    let events = engine.macos_handle_key_event(
        key_code,
        down,
        shift,
        ctrl,
        option,
        meta,
        app_name,
        window_name,
    );
    
    let ffi_events: Vec<FFIKeyEvent> = events
        .into_iter()
        .map(|e| FFIKeyEvent {
            key: CString::new(e.key).unwrap().into_raw(),
            state: e.state == KeyState::Down,
            shift: e.shift,
            ctrl: e.ctrl,
            alt: e.alt,
            meta: e.meta,
        })
        .collect();
    
    unsafe { *out_length = ffi_events.len(); }
    
    let ptr = ffi_events.as_ptr() as *mut FFIKeyEvent;
    std::mem::forget(ffi_events);
    ptr
}

#[no_mangle]
pub extern "C" fn pinkytwirl_free_key_events(events: *mut FFIKeyEvent, length: usize) {
    if !events.is_null() {
        unsafe {
            let events = Vec::from_raw_parts(events, length, length);
            for event in events {
                let _ = CString::from_raw(event.key);
            }
        }
    }
}