use anchor_lang::prelude::*;


#[account]
pub struct StakeState{
    pub owner:Pubkey,
    pub mint:Pubkey,
    pub last_update:i64,
    pub bump:u8
}

impl Space for StakeState{
    const INIT_SPACE: usize = 8 + 32*2 +8 + 1;
}