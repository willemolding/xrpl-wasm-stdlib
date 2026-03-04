#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::host::trace::trace;

#[unsafe(no_mangle)]
pub extern "C" fn on_deposit() -> i32 {
    let _ = trace("Hello Deposit!");

    1 // <-- Allow deposit to succeed by returning a non-zero value
}

#[unsafe(no_mangle)]
pub extern "C" fn on_withdraw() -> i32 {
    let _ = trace("Hello Withdraw!");

    1 // <-- Allow withdrawal to succeed by returning a non-zero value
}
