use pinocchio::{AccountView, Address, ProgramResult};

use crate::error::AxisCoreError;

#[inline(never)]
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Err(AxisCoreError::ScaffoldOnly.into())
}
