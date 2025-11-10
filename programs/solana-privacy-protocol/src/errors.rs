use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid zero-knowledge proof.")]
    InvalidProof,
    #[msg("Double spend detected.")]
    DoubleSpend,
}
