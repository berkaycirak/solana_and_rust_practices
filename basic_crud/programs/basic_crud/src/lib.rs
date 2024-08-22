use anchor_lang::prelude::*;

declare_id!("AGSuh1DgECLSSg4Kh2VhCpaMUF4i4aNYDYJeZUF5Y89U");

#[program]
pub mod basic_crud {
    use super::*;

    pub fn create(ctx: Context<Create>,name:String,age:u8) -> Result<()> {
        ctx.accounts.new_account.set_inner(NewAccount { 
            name, 
            age, 
            bump: ctx.bumps.new_account });

        msg!("Your account is saved: {:?}", ctx.accounts.new_account.key());
        Ok(())
    }
    pub fn update(ctx: Context<Update>,name:String,age:u8) -> Result<()> {
       ctx.accounts.new_account.age = age;
       ctx.accounts.new_account.name = name; 

        msg!("Your account is updated: {:?}", ctx.accounts.new_account.key());
        Ok(())
    }
    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        let data =&mut ctx.accounts.new_account;
        data.close(ctx.accounts.signer.to_account_info())?;
        msg!("Your account is deleted: {:?}", ctx.accounts.new_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer=signer,
        space= NewAccount::INIT_SPACE,
        seeds=[b"data",signer.key.as_ref()],
        bump
    )]
    pub new_account:Account<'info,NewAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>
}
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds=[b"data",signer.key.as_ref()],
        bump=new_account.bump,
    )]
    pub new_account:Account<'info,NewAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>

}
#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(
        mut,
        seeds=[b"data",signer.key.as_ref()],
        bump=new_account.bump,
    )]
    pub new_account:Account<'info,NewAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>

}

#[account]
pub struct NewAccount{
    pub name:String,
    pub age:u8,
    pub bump:u8

}

impl Space for NewAccount{
    const INIT_SPACE: usize = 8 + (4+50) + 1 + 1; // discriminator + name(4 + expected max length) + age + bump
} 







