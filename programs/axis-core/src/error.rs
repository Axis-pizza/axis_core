use pinocchio::error::ProgramError;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AxisCoreError {
    ScaffoldOnly = 1,
}

impl From<AxisCoreError> for ProgramError {
    fn from(error: AxisCoreError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
