use anchor_lang::{prelude::*, system_program::{transfer,Transfer}};

declare_id!("6TA44HKESCTS5VyeJk8fyDGWDyT69hMhbryz4ibUehDp");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Payment>,amount:u64) -> Result<()> {
       ctx.accounts.deposit(amount)?;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Payment>,amount:u64) -> Result<()> {
       ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>)->Result<()>{
        ctx.accounts.close()?;
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

// You need to close account when it is done by using system_program
#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
    mut,
    seeds = [b"vault",vault_state.key().as_ref()],
    bump= vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close=user,
        seeds=[b"state",user.key().as_ref()],
        bump=vault_state.state_bump)]
    pub vault_state:Account<'info,VaultState>,
    pub system_program: Program<'info,System>
}

// impl helps us to access Close struct informations and create methods for our struct.(like java synta x)
impl<'info> Close<'info> {
    pub fn close(&mut self)->Result<()>{
        let cpi_program:AccountInfo=self.system_program.to_account_info();
        let cpi_accounts = Transfer {
             from:self.vault.to_account_info(),
             to:self.user.to_account_info(),
        };
        // While withdrawing, we don't expect user to sign the transaction. We expect it from our vault so we need seeds for our account.
        let seeds:&[&[u8];3] = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds: &[&[&[u8]];1]= &[&seeds[..]];
        // Like in our vault context, we should create a new context for cpi. By sending seeds, cpi check if the our program has ownership of pdas.
        let cpi_ctx:CpiContext<Transfer> = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);
        // find the all lamports in the vault to transfer to user.
        let amount = self.vault.lamports();
        transfer(cpi_ctx,amount)?;

        Ok(())
           
    }
}

impl<'info> Payment<'info> {
    pub fn withdraw(&mut self,amount:u64)->Result<()>{
        //Since transfer is handled by system_program (not from our program), we have to use cross program invocation (cpi).
        let cpi_program:AccountInfo=self.system_program.to_account_info();

        
        let cpi_accounts = Transfer {
             from:self.vault.to_account_info(),
             to:self.user.to_account_info(),
        };
        // While withdrawing, we don't expect user to sign the transaction. We expect it from our vault so we need seeds for our account.
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
