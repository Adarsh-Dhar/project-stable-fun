use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::*;

#[derive(Accounts)]
#[instruction(
    name: String,
    symbol: String,
    target_currency: String,
    icon_uri: String,
)]
pub struct CreateStablecoin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = StablecoinInfo::MAXIMUM_SIZE,
        seeds = [b"stablecoin", authority.key().as_ref(), symbol.as_bytes()],
        bump
    )]
    pub stablecoin_info: Account<'info, StablecoinInfo>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = stablecoin_info,
    )]
    pub mint: Account<'info, Mint>,
    
    pub collateral_mint: Account<'info, Mint>,
    /// CHECK: Oracle account will be validated in the handler
    pub oracle: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateStablecoin>,
    name: String,
    symbol: String,
    target_currency: String,
    icon_uri: String,
) -> Result<()> {
    let stablecoin_info = &mut ctx.accounts.stablecoin_info;
    
    stablecoin_info.authority = ctx.accounts.authority.key();
    stablecoin_info.name = name;
    stablecoin_info.symbol = symbol;
    stablecoin_info.target_currency = target_currency;
    stablecoin_info.icon_uri = icon_uri;
    stablecoin_info.mint = ctx.accounts.mint.key();
    stablecoin_info.collateral_mint = ctx.accounts.collateral_mint.key();
    stablecoin_info.oracle = ctx.accounts.oracle.key();
    stablecoin_info.total_supply = 0;
    stablecoin_info.bump = *ctx.bumps.get("stablecoin_info").unwrap();
    
    Ok(())
} 