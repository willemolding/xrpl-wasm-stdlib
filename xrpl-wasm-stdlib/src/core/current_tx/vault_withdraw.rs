//! # VaultWithdraw
//!
//! This module provides functionality for handling VaultWithdraw transactions within the
//! XRPL Programmability environment.

use crate::core::current_tx::traits::{TransactionCommonFields, VaultWithdrawFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct VaultWithdraw;

/// Implementation of common transaction fields for VaultWithdraw transactions.
///
/// This implementation provides access to standard XRPL transaction fields that are
/// present in all transaction types, such as Account, Fee, Sequence, and others.
/// The methods are provided by the `TransactionCommonFields` trait.
impl TransactionCommonFields for VaultWithdraw {}

/// Implementation of VaultWithdraw-specific transaction fields.
impl VaultWithdrawFields for VaultWithdraw {}

/// Creates a new VaultWithdraw transaction handler for the current transaction context.
///
/// This function returns an `VaultWithdraw` instance that can be used to access fields
/// from the current XRPL transaction. The function assumes that the current transaction
/// is indeed an VaultWithdraw transaction - using this with other transaction types
/// may result in unexpected behavior or errors when accessing type-specific fields.
///
/// # Returns
///
/// Returns an `VaultWithdraw` struct that provides access to both common transaction
/// fields and VaultWithdraw-specific fields through its trait implementations.
///
/// # Safety
///
/// This function is safe to call, but the returned object should only be used when
/// the current transaction context is guaranteed to be an VaultWithdraw transaction.
/// The XRPL Programmability environment ensures this context is correct when the
/// smart contract is invoked in response to an VaultWithdraw transaction.
///
/// # Performance
///
/// This function has zero runtime cost as it simply returns a zero-sized type.
/// All actual field access happens lazily when trait methods are called.
///
/// ```
#[inline]
pub fn get_current_vault_withdraw() -> VaultWithdraw {
    VaultWithdraw
}
