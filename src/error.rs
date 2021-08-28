use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("No rent exemption")]
    NotRentExempt,
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,
    #[error("amount overflow")]
    AmountOverflow,
}