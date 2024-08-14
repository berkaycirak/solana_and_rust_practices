use anchor_lang::prelude::*;
use crate::state::UserState;


#[derive(Accounts)]
pub struct UserInitialize<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        seeds=[b"user".as_ref(),user.key().as_ref()],
        bump,
        space=UserState::INIT_SPACE
    )]
    pub user_account: Account<'info,UserState>,
    pub system_program:Program<'info,System>
}

impl<'info> UserInitialize<'info> {
    pub fn init_user(&mut self,bumps:&UserInitializeBumps)->Result<()>{
        self.user_account.set_inner(UserState { 
            points:0, 
            amount_staked:0, 
            bump:bumps.user_account });

            Ok(())
    }
    
}