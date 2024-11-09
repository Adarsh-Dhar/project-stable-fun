use anchor_lang::prelude::*;

#[error_code]
pub enum StableFunError {
    #[msg("Invalid oracle price")]
    InvalidOraclePrice,
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Math overflow")]
    MathOverflow,
} 