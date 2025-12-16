#![allow(non_camel_case_types)]
//! Minimal example C-ABI Tome plugin using the shared tome-cabi-types crate.

use std::ffi::CString;

use tome_cabi_types::{TomeGuestV1, TomeHostV1, TomeStatus, TOME_C_ABI_VERSION};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tome_plugin_entry(host: *const TomeHostV1, out_guest: *mut TomeGuestV1) -> TomeStatus {
    if out_guest.is_null() || host.is_null() {
        return TomeStatus::Failed;
    }

    let host_ref = unsafe { &*host };
    if host_ref.abi_version != TOME_C_ABI_VERSION {
        return TomeStatus::Incompatible;
    }

    unsafe {
        HOST_LOG = host_ref.log;
        *out_guest = TomeGuestV1 { abi_version: TOME_C_ABI_VERSION, init: Some(plugin_init) };
    }
    TomeStatus::Ok
}

extern "C" fn plugin_init() -> TomeStatus {
    if let Some(log) = unsafe { HOST_LOG } {
        if let std::result::Result::Ok(msg) = CString::new("hello from cabi demo plugin") {
            log(msg.as_ptr());
        }
    }
    TomeStatus::Ok
}

// SAFETY: set when entry is called; only used in plugin_init.
static mut HOST_LOG: Option<extern "C" fn(*const core::ffi::c_char)> = None;
