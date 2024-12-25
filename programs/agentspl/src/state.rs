use anchor_lang::prelude::*;

#[account]
pub struct TokenState {
    pub authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
}

#[account]
pub struct CompressionConfig {
    pub max_depth: u32,
    pub max_buffer_size: u32,
    pub authority: Pubkey,
}