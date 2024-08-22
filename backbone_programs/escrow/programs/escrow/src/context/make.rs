use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,token_interface::{TokenInterface,TokenAccount,Mint,TransferChecked,transfer_checked}};

use crate::Escrow;



#[derive(Accounts)] 
#[instruction(seed:u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker:Signer<'info>,
    #[account(
        mint::token_program=token_program
    )]
    mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    mint_b:Box<InterfaceAccount<'info,Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_a:Box<InterfaceAccount<'info,TokenAccount>>,
    #[account(
        init,
        payer=maker,
        space= 8 + Escrow::INIT_SPACE,
        seeds= [b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow:Account<'info,Escrow>,
    #[account(
        init_if_needed,
        payer=maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow, // our escrow will have an authority for that vault
        associated_token::token_program = token_program,
    )]
    vault:InterfaceAccount<'info,TokenAccount>,
    associated_token_program:Program<'info,AssociatedToken>, // Calculates ATAs on the accounts
    token_program: Interface<'info,TokenInterface>, // Doing token transfer, open mints or etc.
    system_program:Program<'info,System> // For opening escrow account state
}

impl<'info> Make <'info> {
    // initialization
    pub fn initialize(&mut self,seed:u64,receive:u64,bump:u8)->Result<()>{
        self.escrow.set_inner(Escrow{
            seed,
            maker:self.maker.key(),
            mint_a:self.mint_a.key(),
            mint_b:self.mint_b.key(),
            receive,
            bump,
        });
        Ok(())
    }

    pub fn deposit_to_vault(&mut self,amount:u64)->Result<()>{
        // We need to check some accounts for transfer
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