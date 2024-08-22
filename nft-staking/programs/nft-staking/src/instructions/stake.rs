use anchor_lang::prelude::Signer;
use anchor_spl::token::TokenAccount;



#[derive(Accounts)]
pub struct Stake<'info>{
#[account(mut)]
pub user:Signer<'info>,
pub mint:Account<'info,Mint>,
pub collection: Account<'info,Mint>,
#[account(
    mut,
associated_token::mint=mint,
associated_token::authority=user,

)]
pub mint_ata:Account<'info,TokenAccount>,

pub metadata:Account<'info,MetadataAccount>

}