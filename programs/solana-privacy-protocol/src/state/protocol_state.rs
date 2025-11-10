use anchor_lang::prelude::*;

/// Global protocol state
#[account]
#[derive(Default)]
pub struct ProtocolState {
    pub authority: Pubkey,
    pub state_bump: u8,
    pub vault_bump: u8,
    pub total_shielded: u64,
    pub total_public: u64,
}

impl ProtocolState {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8;
}
