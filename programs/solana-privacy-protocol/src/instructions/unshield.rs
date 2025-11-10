use anchor_lang::prelude::*;
use crate::state::{ProtocolState, PrivacyPool};

/// Handles unshielding (Private → Public)
/// Prevents double spend by checking nullifier existence
pub fn unshield_tokens(
    ctx: Context<UnshieldTokens>,
    amount: u64,
    nullifier: [u8; 32],
) -> Result<()> {
    let state = &mut ctx.accounts.protocol_state;
    let pool = &mut ctx.accounts.privacy_pool;

    // Prevent double spend
    require!(
        !pool.is_nullifier_used(&nullifier),
        ErrorCode::DoubleSpend
    );

    // Record nullifier
    pool.nullifiers.push(nullifier);

    // Update total public balance
    state.total_public = state.total_public.saturating_add(amount);

    msg!(
        "Unshielded {} tokens from privacy pool by {}",
        amount,
        ctx.accounts.authority.key()
    );

    Ok(())
}

#[derive(Accounts)]
pub struct UnshieldTokens<'info> {
    #[account(mut, has_one = authority)]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub privacy_pool: Account<'info, PrivacyPool>,
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Double spend detected")]
    DoubleSpend,
}
