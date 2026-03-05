use crate::core::ledger_objects::traits::{CurrentLedgerObjectCommonFields, CurrentVaultFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CurrentVault;

impl CurrentLedgerObjectCommonFields for CurrentVault {}

impl CurrentVaultFields for CurrentVault {}

#[inline]
pub fn get_current_escrow() -> CurrentVault {
    CurrentVault
}
