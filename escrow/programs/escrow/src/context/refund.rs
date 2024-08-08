use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::CloseAccount, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::Escrow;



#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Refund<'info> {
    #[account(mut)]
    maker:Signer<'info>,
    #[account(
        mint::token_program=token_program
    )]
    mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    mint_b:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_a:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        close=maker,
        seeds= [b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow:Account<'info,Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    vault:InterfaceAccount<'info,TokenAccount>,
    associated_token_program:Program<'info,AssociatedToken>,
    token_program: Interface<'info,TokenInterface>,
    system_program:Program<'info,System>
}

impl<'info> Refund <'info> {
    pub fn withdraw_and_close(&mut self,seed:u64,receive:u64,bump:u8)->Result<()>{
        let seed:self.escrow.seed.to_le_bytes();
        let bump:[self.escrow.bump];
        let signer_seeds: [&[&[&[u8];3];1];1] = &[&[b"escrow",self.maker.to_account_info().key().as_ref(),&seed.as_ref(),bump]]
        let accounts = TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.maker_ata_a.to_account_info(),
            authority:self.escrow.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts,&signer_seeds);

        transfer_checked(ctx, self.amount.vault, self.mint_a.decimals);

        let accounts = CloseAccount{
            account:self.vault.to_account_info(),
            
        }
        Ok(())
    }

    pub fn deposit_to_vault(&mut self,amount:u64)->Result<()>{
        let accounts = TransferChecked{
            from:self.maker_ata_a.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info()
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, self.mint_a.decimals)
    }
}