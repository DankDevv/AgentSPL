use anchor_lang::prelude::*;

#[error_code]
pub enum AgentSPLError {
    #[msg("Token account not initialized")]
    NotInitialized,
    #[msg("Invalid token authority")]
    InvalidAuthority,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
}