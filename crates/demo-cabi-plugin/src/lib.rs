#![allow(non_camel_case_types)]
//! Minimal example C-ABI Tome plugin. No dependencies besides the standard library.

use std::ffi::{c_char, CString};

const TOME_C_ABI_VERSION: u32 = 1;

type TomeStatus = i32;
const TOME_STATUS_OK: TomeStatus = 0;
const TOME_STATUS_FAILED: TomeStatus = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TomeHostV1 {
    pub abi_version: u32,
    pub log: Option<extern "C" fn(*const c_char)>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TomeGuestV1 {
    pub abi_version: u32,
    pub init: Option<extern "C" fn() -> TomeStatus>,
}

#[no_mangle]
pub unsafe extern "C" fn tome_plugin_entry(host: *const TomeHostV1, out_guest: *mut TomeGuestV1) -> TomeStatus {
    if out_guest.is_null() || host.is_null() {
        return TOME_STATUS_FAILED;
    }

    let host_ref = &*host;
    if host_ref.abi_version != TOME_C_ABI_VERSION {
        return TOME_STATUS_FAILED;
    }

    unsafe {
        HOST_LOG = host_ref.log;
    }

    *out_guest = TomeGuestV1 { abi_version: TOME_C_ABI_VERSION, init: Some(plugin_init) };
    TOME_STATUS_OK
}

extern "C" fn plugin_init() -> TomeStatus {
    // Demonstrate logging back into the host.
    if let Some(log) = unsafe { HOST_LOG } {
        let msg = CString::new("hello from cabi demo plugin").unwrap();
        log(msg.as_ptr());
    }
    TOME_STATUS_OK
}

// SAFETY: set when entry is called; only used in plugin_init.
static mut HOST_LOG: Option<extern "C" fn(*const c_char)> = None;
