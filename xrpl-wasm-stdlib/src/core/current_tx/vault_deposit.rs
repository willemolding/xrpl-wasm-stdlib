//! # VaultDeposit
//!
//! This module provides functionality for handling VaultDeposit transactions within the
//! XRPL Programmability environment.

use crate::core::current_tx::traits::{TransactionCommonFields, VaultDepositFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct VaultDeposit;

/// Implementation of common transaction fields for VaultDeposit transactions.
///
/// This implementation provides access to standard XRPL transaction fields that are
/// present in all transaction types, such as Account, Fee, Sequence, and others.
/// The methods are provided by the `TransactionCommonFields` trait.
impl TransactionCommonFields for VaultDeposit {}

/// Implementation of VaultDeposit-specific transaction fields.
impl VaultDepositFields for VaultDeposit {}

/// Creates a new VaultDeposit transaction handler for the current transaction context.
///
/// This function returns an `VaultDeposit` instance that can be used to access fields
/// from the current XRPL transaction. The function assumes that the current transaction
/// is indeed an VaultDeposit transaction - using this with other transaction types
/// may result in unexpected behavior or errors when accessing type-specific fields.
///
/// # Returns
///
/// Returns an `VaultDeposit` struct that provides access to both common transaction
/// fields and VaultDeposit-specific fields through its trait implementations.
///
/// # Safety
///
/// This function is safe to call, but the returned object should only be used when
/// the current transaction context is guaranteed to be an VaultDeposit transaction.
/// The XRPL Programmability environment ensures this context is correct when the
/// smart contract is invoked in response to an VaultDeposit transaction.
///
/// # Performance
///
/// This function has zero runtime cost as it simply returns a zero-sized type.
/// All actual field access happens lazily when trait methods are called.
///
/// ```
#[inline]
pub fn get_current_vault_deposit() -> VaultDeposit {
    VaultDeposit
}
