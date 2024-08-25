use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount,transfer,Transfer}};

declare_id!("GAB2jMEAr6qTsPgEdCBuuKtVgqmh1nTntsrbzcDYcFzW");

#[program]
pub mod custom_bank {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {

        ctx.accounts.bank_account.set_inner(BankAccountState { 
            owner:ctx.accounts.owner.key(), 
            token: ctx.accounts.token.key(), 
            bank_ata: ctx.accounts.bank_token_account.key(), 
            balance: 0, 
            bump: ctx.bumps.bank_account
         });
        msg!("Your bank account is created: {:?}", ctx.accounts.bank_account.key());
        Ok(())
    }

    pub fn deposit(ctx:Context<Deposit>,amount:u64) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;
        // Send money from owner_token_account --> bank_token_account
        let owner_ata = ctx.accounts.owner_token_account.to_account_info();
        let bank_ata = ctx.accounts.bank_token_account.to_account_info();

        // Cross Program Invocation (CPI) is required for interacting with spl token program to transfer in our program.

        let cpi_accounts = Transfer{
            from:owner_ata,
            to:bank_ata,
            authority:ctx.accounts.owner.to_account_info()
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        bank_account.balance+=amount;
        // Check for add

        let _ = bank_account.balance.checked_add(amount);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn withdraw(ctx:Context<Withdraw>,amount:u64) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;
        // Send money from owner_token_account --> bank_token_account
        let owner_ata = ctx.accounts.owner_token_account.to_account_info();
        let bank_ata = ctx.accounts.bank_token_account.to_account_info();

        // Cross Program Invocation (CPI) is required for interacting with spl token program to transfer in our program.

        let cpi_accounts = Transfer{
            from:bank_ata,
            to:owner_ata,
            authority:bank_account.to_account_info()
        };

        let seeds = &[
            &b"bank"[..],
            &ctx.accounts.owner.key().to_bytes(),
            &ctx.accounts.token.key().to_bytes(),
            &[bank_account.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts,signer_seeds);
        bank_account.balance-=amount;
        // check for amount whether bigger than balance or not
        let _ = bank_account.balance.checked_sub(amount);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn close_vault(ctx:Context<CloseVault>) -> Result<()> {

        // For safe closing, you should check owner ATA, if it has some balance, you should transfer that balance to the owner ATA before
        let bank_account = &mut ctx.accounts.bank_account;
        let owner = ctx.accounts.owner.to_account_info();

        bank_account.close(owner)?;
        Ok(())
    }

    


}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        init,
        payer=owner,
        space=BankAccountState::INIT_SPACE,
        seeds=[b"bank",owner.key().as_ref(),token.key().as_ref()],
        bump

    )]
    pub bank_account:Account<'info,BankAccountState>,
    #[account(
        init,
        payer=owner,
        associated_token::mint = token,
        associated_token::authority = bank_account
    )]
    pub bank_token_account:Account<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token,
        associated_token::authority=owner
    )]
    pub owner_token_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub token:Account<'info,Mint>,
    pub system_program:Program<'info,System>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token> 
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        mut,
        seeds=[b"bank",owner.key().as_ref(),token.key().as_ref()],
        bump

    )]
    pub bank_account:Account<'info,BankAccountState>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = bank_account
    )]
    pub bank_token_account:Account<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token,
        associated_token::authority=owner
    )]
    pub owner_token_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub token:Account<'info,Mint>,
    pub system_program:Program<'info,System>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token> 
}
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        mut,    
        seeds=[b"bank",owner.key().as_ref(),token.key().as_ref()],
        bump

    )]
    pub bank_account:Account<'info,BankAccountState>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = bank_account
    )]
    pub bank_token_account:Account<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token,
        associated_token::authority=owner
    )]
    pub owner_token_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub token:Account<'info,Mint>,
    pub system_program:Program<'info,System>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token> 
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        mut,
        seeds=[b"bank",owner.key().as_ref(),token.key().as_ref()],
        bump

    )]
    pub bank_account:Account<'info,BankAccountState>,
    pub system_program:Program<'info,System>,
    #[account(mut)]
    pub token:Account<'info,Mint>,
    
}

#[account]
pub struct BankAccountState {
    pub owner:Pubkey, // Signer |  That is for protection
    pub token:Pubkey, // Mint Address 
    pub bank_ata:Pubkey, // Your Token ATA in the Bank Account
    pub balance:u64, // Counter for related token balance
    pub bump:u8 // to find PDA in which that token state is stored easily.

}

impl Space for BankAccountState {
    const INIT_SPACE: usize = 8 + (3 * 32) + 8 + 1 ;
}
