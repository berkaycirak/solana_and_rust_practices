use anchor_lang::prelude::*;

mod context;
use context::*;
mod state;
use state::*;


declare_id!("U7pd4JY2NVmiaha9yUBwYKV3BQY8J9Wdc4YpNhzVSDA");

#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx:Context<Make>,seed:u64,amount:u64,receive:u64)->Result<()>{
        ctx.accounts.initialize(seed,receive,ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)?;
        Ok(())
    }
    pub fn take_and_close(ctx:Context<Take>)->Result<()>{
        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.take_and_close()?;
        Ok(())
    }

    pub fn refund(ctx:Context<Refund>)->Result<()>{
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
