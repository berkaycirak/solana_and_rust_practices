use anchor_lang::prelude::*;


#[account]

pub struct Config{
    pub points_per_stake:u8, // since there will be multiple staking
    pub max_stake:u8,
    pub freeze_period:u32,
    pub rewards_bump:u8,
    pub bump:u8
}

impl Space for Config{
    const INIT_SPACE: usize = 8 + 1*4 + 4;
}