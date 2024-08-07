use anchor_lang::prelude::*;

declare_id!("6TA44HKESCTS5VyeJk8fyDGWDyT69hMhbryz4ibUehDp");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
