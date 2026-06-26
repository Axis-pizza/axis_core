use litesvm::LiteSVM;
use solana_address::Address;

use crate::{accounts::account_data, AxisTestError, Result};

const TOKEN_ACCOUNT_MIN_LEN: usize = 165;
const TOKEN_ACCOUNT_AMOUNT_OFFSET: usize = 64;
const TOKEN_ACCOUNT_AMOUNT_LEN: usize = 8;
const TOKEN_ACCOUNT_STATE_OFFSET: usize = 108;
const TOKEN_ACCOUNT_UNINITIALIZED_STATE: u8 = 0;

/// Token-2022 extensions append data after the canonical 165-byte token account
/// base layout, so this helper only inspects the shared amount field.
pub fn token_account_balance(vm: &LiteSVM, token_account: &Address) -> Result<u64> {
    let data = account_data(vm, token_account)?;
    token_account_balance_from_data(token_account, &data)
}

pub fn token_account_balance_from_data(address: &Address, data: &[u8]) -> Result<u64> {
    if data.len() < TOKEN_ACCOUNT_MIN_LEN {
        return Err(AxisTestError::InvalidTokenAccountData {
            address: address.clone(),
            reason: format!(
                "expected at least {TOKEN_ACCOUNT_MIN_LEN} bytes, got {} bytes",
                data.len()
            ),
        });
    }

    if data[TOKEN_ACCOUNT_STATE_OFFSET] == TOKEN_ACCOUNT_UNINITIALIZED_STATE {
        return Err(AxisTestError::InvalidTokenAccountData {
            address: address.clone(),
            reason: "token account state is uninitialized".to_string(),
        });
    }

    let amount_end = TOKEN_ACCOUNT_AMOUNT_OFFSET + TOKEN_ACCOUNT_AMOUNT_LEN;
    let amount = data[TOKEN_ACCOUNT_AMOUNT_OFFSET..amount_end]
        .try_into()
        .expect("token amount slice length is fixed");

    Ok(u64::from_le_bytes(amount))
}
