use anchor_lang::prelude::*;

mod state;
use state::*;

mod instructions;
use instructions::*;
declare_id!("41TqYMSv7j65zTrSiXY6aSEsRQGs23SVgizLXWKGx7Ry");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,) -> Result<()> {
        ctx.accounts.init();
        Ok(());
    }

    pub fn create_user(ctx:Context<UserInitialize>)->Result<()>{
        ctx.accounts.init_user()
    }
}





