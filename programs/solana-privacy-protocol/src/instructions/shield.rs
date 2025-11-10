use anchor_lang::prelude::*;
use crate::state::{ProtocolState, PrivacyPool};

pub fn shield_tokens(ctx: Context<ShieldTokens>, amount: u64, commitment: [u8; 32]) -> Result<()> {
    let state = &mut ctx.accounts.protocol_state;
    let pool = &mut ctx.accounts.privacy_pool;

    // Add leaves to the dummy tree
    pool.tree.append(commitment)?;

    // Shielded token statistics update
    state.total_shielded = state.total_shielded.saturating_add(amount);

    msg!("Shielded {} tokens successfully", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct ShieldTokens<'info> {
    #[account(mut)]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub privacy_pool: Account<'info, PrivacyPool>,
    pub authority: Signer<'info>,
}
