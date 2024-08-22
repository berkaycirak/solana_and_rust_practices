use anchor_lang::prelude::*;

#[account]
pub struct UserState {
    pub points:u32,
    pub amount_staked:u8,
    pub bump:u8,
}

impl Space for UserState{
    const INIT_SPACE: usize = 8 + 1*2 + 4;
}