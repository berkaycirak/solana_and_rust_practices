use anchor_lang::{prelude::*, solana_program::lamports, system_program::{transfer,Transfer}};

declare_id!("6TA44HKESCTS5VyeJk8fyDGWDyT69hMhbryz4ibUehDp");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state",user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info,VaultState>,
    #[account(
        seeds = [b"vault",vault_state.key().as_ref()],bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program:Program<'info,System>,
}



impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps:&InitializeBumps)->Result<()>{
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault:SystemAccount<'info>,
    #[account(
        seeds = [b"state",user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state:Account<'info,VaultState>,
    // System program needs for doing transfer
    pub system_program:Program<'info,System>,
}

impl<'info> Payment<'info> {
    pub fn withdraw(&mut self,amount:u64)->Result<()>{
        let cpi_program:AccountInfo=self.system_program.to_account_info();

        let cpi_accounts = Transfer {
             from:self.user.to_account_info(),
             to:self.vault.to_account_info(),
        };

        let seeds:&[&[u8];3] = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds: &[&[&[u8]];1]= &[&seeds[..]];
        // Like in our vault context, we should create a new context for cpi.
        let cpi_ctx:CpiContext<Transfer> = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);
        transfer(cpi_ctx,amount)?;

        Ok(())
           
    }

    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let cpi_program:AccountInfo=self.system_program.to_account_info();

        let cpi_accounts = Transfer {
             from:self.user.to_account_info(),
             to:self.vault.to_account_info(),
        };
        // Like in our vault context, we should create a new context for cpi.
        let cpi_ctx:CpiContext<Transfer> = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx,amount)?;

        Ok(())
           
    }
}

#[account]
pub struct VaultState{
    pub vault_bump:u8,
    pub state_bump:u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1; // anchor discriminator, score, bump 
}
