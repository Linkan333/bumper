use anchor_lang::prelude::*;

declare_id!("BcjvsTmAYLfKVvayMbQn1oJRDBvNTzpsyTYYj4GMsBSc");

#[program]
pub mod bump_bot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
