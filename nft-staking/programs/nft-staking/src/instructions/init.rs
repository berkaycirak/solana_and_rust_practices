use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::Config;

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,
    #[account(
        init,
        payer=admin,
        seeds=[b"config".as_ref()],
        bump,
        space= Config::INIT_SPACE
    )]
    pub config:Account<'info,Config>,

    #[account(
      
        init,
        payer=admin,
        seeds=[b"rewards".as_ref(),config.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config

    )]
    pub rewards_mint:Account<'info,Mint>,
    pub system_program:Program<'info,System>, // for init an account
    pub token_program:Program<'info,Token> // for creating token account (we can think it is not a system account but token account), doing transfers
}

impl<'info> Initialize<'info>{
    pub fn init(&mut self,points_per_stake:u8,max_stake:u8,freeze_period:u32,bumps:&InitializeBumps){
        // You can set all properties by using set_inner
      self.config.set_inner(Config {
         points_per_stake, 
         max_stake, 
         freeze_period, 
         rewards_bump:bumps.rewards_mint, 
         bump:bumps.config
        })
    }
}