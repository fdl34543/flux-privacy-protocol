use anchor_lang::prelude::*;

/// For future use — privacy staking feature
#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub privacy_score: u32,
}

impl StakeAccount {
    pub const LEN: usize = 32 + 8 + 4;
}
