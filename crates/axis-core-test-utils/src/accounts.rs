use litesvm::LiteSVM;
use solana_address::Address;

use crate::{AxisTestError, Result};

pub fn account_data(vm: &LiteSVM, address: &Address) -> Result<Vec<u8>> {
    vm.get_account(address)
        .map(|account| account.data)
        .ok_or_else(|| AxisTestError::AccountNotFound {
            address: address.clone(),
        })
}

pub fn account_lamports(vm: &LiteSVM, address: &Address) -> Result<u64> {
    vm.get_balance(address)
        .ok_or_else(|| AxisTestError::AccountNotFound {
            address: address.clone(),
        })
}
