use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,CloseAccount,close_account}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Take<'info> {
    #[account(mut)]
    maker:SystemAccount<'info>,
    #[account(mut)]
    taker:Signer<'info>,
    #[account(
        mint::token_program=token_program
    )]
    mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    mint_b:InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    taker_ata_a:InterfaceAccount<'info,TokenAccount>, // We need taker_ata_a , since taker will receive funds from maker_ata_a. In case of no account, we should init_if_needed
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    taker_ata_b:InterfaceAccount<'info,TokenAccount>, // Since taker will give its mint_b to the maker_ata_a, we need this.
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_b:InterfaceAccount<'info,TokenAccount>, // maker_ata_b will receive mint_a from taker. Therefore, we need that account. In case of no account, init_if_needed.
    
    #[account(
        mut,
        close=taker,
        seeds= [b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow:Account<'info,Escrow>,// Since we initialize in the make mod that escrow, we don't need to init here. Instead, we should close it after take. And taker will get the rent for the closed account.
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

impl<'info> Take<'info>{
    // Since taker will pay for receiving makers mint_a token, taker must deposits its mint_b to the maker. We skip vault in that case. We don't need account in that case as escrow itself will handle for amount.
    pub fn deposit_to_maker(&mut self)->Result<()>{
        // We need to check some accounts for transfer
        let accounts = TransferChecked{
            from:self.taker_ata_b.to_account_info(),
            mint:self.mint_b.to_account_info(),
            to:self.maker_ata_b.to_account_info(),
            authority:self.taker.to_account_info()
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, self.escrow.receive, self.mint_b.decimals)
    }

    pub fn take_and_close(&mut self)->Result<()>{
        let seed=self.escrow.seed.to_le_bytes();
        let bump=[self.escrow.bump];
        let signer_seeds= [&[b"escrow",self.maker.to_account_info().key.as_ref(),&seed.as_ref(),&bump][..]];

        let accounts = TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.taker_ata_a.to_account_info(),
            authority:self.escrow.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts,&signer_seeds);

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?;

        // After transferring all amount in vault, we should close the account since we don't need it anymore. We can reuse same variable name, since rust allows shadowing.
        let accounts = CloseAccount{
            account:self.vault.to_account_info(),
            authority:self.escrow.to_account_info(),
            destination:self.taker.to_account_info(),
            
        };
        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts,&signer_seeds);

        close_account(ctx)?;

        Ok(())
    }
}