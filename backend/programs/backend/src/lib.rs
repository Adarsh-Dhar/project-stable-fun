use anchor_lang::prelude::*;

declare_id!("Ewix8ZYPtptBKtQnXAT2kv2ergWZsJpDRh1UykVkHZEo");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
