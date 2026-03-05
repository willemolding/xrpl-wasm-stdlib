#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::core::current_tx::escrow_finish;
use xrpl_wasm_stdlib::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_stdlib::host::trace::trace_num;
use xrpl_wasm_stdlib::host::{Result::Err, Result::Ok};
use xrpl_wasm_stdlib::r_address;

// These accounts are the only ones allowed to interact with the vault
const WITHDRAW_WHITELIST: [[u8; 20]; 1] = [r_address!("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")];

#[unsafe(no_mangle)]
pub extern "C" fn on_deposit() -> i32 {
    1 // <-- Anyone can deposit, so allow by returning a non-zero value
}

#[unsafe(no_mangle)]
pub extern "C" fn on_withdraw() -> i32 {
    let escrow_finish = escrow_finish::get_current_escrow_finish();
    let tx_account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code(); // Must return to short circuit.
        }
    };

    for acc in WITHDRAW_WHITELIST {
        if tx_account.0 == acc {
            return 1; // <-- Allow withdrawal to succeed by returning a non-zero value
        }
    }
    0 // <-- Deny withdrawal by returning zero
}
