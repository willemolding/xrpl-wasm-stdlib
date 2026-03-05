#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use state::StorageVaultState;
use xrpl_wasm_stdlib::core::{
    current_tx::{
        traits::{TransactionCommonFields, VaultDepositFields, VaultWithdrawFields},
        vault_deposit::get_current_vault_deposit,
        vault_withdraw::get_current_vault_withdraw,
    },
    ledger_objects::{
        current_vault::{CurrentVault, get_current_vault},
        traits::CurrentVaultFields,
    },
    types::amount::Amount,
};

mod state;

#[unsafe(no_mangle)]
pub extern "C" fn on_deposit() -> i32 {
    let vault_deposit = get_current_vault_deposit();
    let drops = match vault_deposit.get_amount().unwrap() {
        Amount::XRP { num_drops } => num_drops,
        _ => panic!("Vault only supports XRP"),
    };

    // Record the deposit in the vault's state.
    let mut data = get_current_vault().get_data().unwrap();
    let mut state = StorageVaultState::from_bytes(&data.data);

    state.append_record(vault_deposit.get_account().unwrap(), drops);
    state.to_bytes(&mut data.data);
    data.len = StorageVaultState::SERIALIZED_SIZE;
    CurrentVault::update_current_vault_data(data).unwrap();

    1
}

#[unsafe(no_mangle)]
pub extern "C" fn on_withdraw() -> i32 {
    let vault_withdraw = get_current_vault_withdraw();
    let account = vault_withdraw.get_account().unwrap();
    let drops = match vault_withdraw.get_amount().unwrap() {
        Amount::XRP { num_drops } => num_drops,
        _ => panic!("Vault only supports XRP"),
    };

    let mut data = get_current_vault().get_data().unwrap();
    let mut state = StorageVaultState::from_bytes(&data.data);

    if state.remove_record(&account, drops) {
        state.to_bytes(&mut data.data);
        data.len = StorageVaultState::SERIALIZED_SIZE;
        CurrentVault::update_current_vault_data(data).unwrap();
        1
    } else {
        0
    }
}
