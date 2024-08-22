use anchor_lang::prelude::*;

declare_id!("AGSuh1DgECLSSg4Kh2VhCpaMUF4i4aNYDYJeZUF5Y89U");

#[program]
pub mod basic_crud {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}





