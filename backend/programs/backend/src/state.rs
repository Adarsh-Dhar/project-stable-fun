use anchor_lang::prelude::*;

#[account]
pub struct StablecoinInfo {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub target_currency: String,
    pub icon_uri: String,
    pub mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub oracle: Pubkey,
    pub total_supply: u64,
    pub bump: u8,
}

impl StablecoinInfo {
    pub const MAXIMUM_SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + // name
        8 + // symbol
        16 + // target_currency
        128 + // icon_uri
        32 + // mint
        32 + // collateral_mint
        32 + // oracle
        8 + // total_supply
        1; // bump
} 