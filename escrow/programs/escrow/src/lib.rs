use anchor_lang::prelude::*;

mod context;
use context::*;
mod state;
use state::*;


declare_id!("U7pd4JY2NVmiaha9yUBwYKV3BQY8J9Wdc4YpNhzVSDA");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx:Context<Initialize>,seed:u64,amount:u64,receive:u64)->Result<()>{
        ctx.accounts.save_escrow(seed,receive,ctx.bumps.escrow)?;
        // ctx.accounts.deposit_to_vault(amount)
    }
    pub fn take(ctx:Context<Initialize>)->Result<()>{
        // ctx.accounts.transfer()?
        // ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx:Context<Refund>)->Result<()>{
        ctx.accounts.withdraw_and_close();
    }
}

#[derive(Accounts)]
pub struct Initialize {}
