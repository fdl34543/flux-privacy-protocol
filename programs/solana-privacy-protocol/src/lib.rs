use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_lang::Result as AnchorResult;

// Light Protocol SDK (modular structure)
use light_sdk::{
    account::*,
    token::*,
    transfer::*,
    utils::*,
    error::*,
    merkle_tree::v1::*,
};
use crate::state::*;
// Import types & Poseidon from modern SDK packages
// use light_sdk::merkle_tree::v1::ConcurrentMerkleTreeZeroCopy;
use light_sdk::merkle_tree::v1::*;
use light_sdk::light_hasher::Poseidon;
use light_sdk_pinocchio::*;

use crate::state::{ProtocolState, PrivacyPool};

pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("A1QwxxHo4FXq8UumCmqM8P2iexxSjUY3SKGJFB5zwcMY");


#[program]
pub mod solana_privacy_protocol {
    use super::*;

    /// Initialize protocol state and vault accounts
    pub fn initialize(ctx: Context<Initialize>) -> AnchorResult<()> {
        let state = &mut ctx.accounts.protocol_state;
        state.authority = *ctx.accounts.authority.key;
        state.state_bump = ctx.bumps.protocol_state;
        state.vault_bump = ctx.bumps.vault;
        state.total_shielded = 0;
        state.total_public = 0;

        // Initialize PrivacyPool data safely
        {
            let pool_info = ctx.accounts.privacy_pool.to_account_info();
            let mut data = pool_info.try_borrow_mut_data()?;

            // Write discriminator if not written yet
            let disc = <crate::state::PrivacyPool as anchor_lang::Discriminator>::DISCRIMINATOR;
            if data.len() >= 8 && data[0..8] != disc[..] {
                data[0..8].copy_from_slice(&disc);
                msg!("PrivacyPool discriminator written.");
            }

            // Deserialize or default
            let mut pool: crate::state::PrivacyPool =
                anchor_lang::AccountDeserialize::try_deserialize_unchecked(&mut &data[..])
                    .unwrap_or_default();

            // Reset to fresh state
            pool.tree = crate::state::DummyMerkleTree::new();
            pool.nullifiers.clear();
            pool.user_balances.clear();

            // Serialize back
            let mut dst = Vec::new();
            pool.try_serialize(&mut dst)?;
            let len = dst.len().min(data.len());
            data[..len].copy_from_slice(&dst[..len]);
        }

        msg!("✅ Protocol initialized successfully.");
        Ok(())
    }

    /// Shield tokens (public → private)
    pub fn shield_tokens(
        ctx: Context<ShieldTokens>,
        amount: u64,
        commitment: [u8; 32],
    ) -> AnchorResult<()> {
        // Step 1: transfer SPL tokens to vault
        let cpi_ctx = ctx.accounts.transfer_to_vault();
        token::transfer(cpi_ctx, amount)?;

        // Step 2: update dummy Merkle tree in privacy pool
        let pool = &mut ctx.accounts.privacy_pool;
        pool.tree.append(commitment)?;

        // Step 3: update protocol state
        ctx.accounts.protocol_state.total_shielded = ctx
            .accounts
            .protocol_state
            .total_shielded
            .saturating_add(amount);

        // Step 4: update user-specific shielded balance
        if let Some(entry) = pool
            .user_balances
            .iter_mut()
            .find(|(pk, _)| *pk == *ctx.accounts.authority.key)
        {
            entry.1 = entry.1.saturating_add(amount);
        } else {
            pool.user_balances.push((*ctx.accounts.authority.key, amount));
        }

        msg!(
            "Shielded {} tokens successfully for wallet {}",
            amount,
            ctx.accounts.authority.key()
        );

        Ok(())
    }

    /// Unshield tokens (private → public)
    pub fn unshield_tokens(
        ctx: Context<UnshieldTokens>,
        amount: u64,
        nullifier: [u8; 32],
        proof_data: Vec<u8>,
        public_inputs: Vec<u8>,
    ) -> AnchorResult<()> {
        // Step 1: Verify dummy ZK proof
        let verified = verify_zk_proof(&proof_data, &public_inputs)?;
        require!(verified, ErrorCode::InvalidProof);

        // Step 2: Nullifier check (double spend prevention)
        {
            let pool = &mut ctx.accounts.privacy_pool;
            require!(!pool.nullifiers.contains(&nullifier), ErrorCode::DoubleSpend);
            pool.nullifiers.push(nullifier);
        } // <-- borrow of pool ends here ✅

        // Step 3: Transfer tokens from vault to user (safe immutable borrow)
        {
            let cpi_ctx = ctx.accounts.transfer_from_vault()?;
            token::transfer(cpi_ctx, amount)?;
        }

        // Step 4: Update user-specific shielded balance
        {
            let pool = &mut ctx.accounts.privacy_pool;
            if let Some(entry) = pool
                .user_balances
                .iter_mut()
                .find(|(pk, _)| *pk == *ctx.accounts.authority.key)
            {
                entry.1 = entry.1.saturating_sub(amount);
            }
        }

        // Step 5: Update protocol totals
        {
            let state = &mut ctx.accounts.protocol_state;
            state.total_public = state.total_public.saturating_add(amount);
        }

        msg!(
            "Unshielded {} tokens to user {}",
            amount,
            ctx.accounts.authority.key()
        );

        Ok(())
    }

    pub fn private_transfer(
        ctx: Context<PrivateTransfer>,
        old_nullifier: [u8; 32],
        new_commitment: [u8; 32],
        proof_data: Vec<u8>,
        public_inputs: Vec<u8>,
    ) -> AnchorResult<()> {
        // 1️⃣ Verify proof (dummy for now)
        let verified = verify_zk_proof(&proof_data, &public_inputs)?;
        require!(verified, ErrorCode::InvalidProof);

        let pool = &mut ctx.accounts.privacy_pool;

        // 2️⃣ Prevent double-spend
        require!(
            !pool.nullifiers.contains(&old_nullifier),
            ErrorCode::DoubleSpend
        );
        pool.nullifiers.push(old_nullifier);

        // 3️⃣ Append new commitment (receiver’s note)
        pool.tree.append(new_commitment)?;

        // 4️⃣ Optional logging
        msg!(
            "Private transfer executed: old_nullifier {:?}, new_commitment {:?}",
            &old_nullifier[..4],
            &new_commitment[..4]
        );

        Ok(())
    }


    // --- Force IDL export ---
    pub fn idl_expose_privacy_pool(_ctx: Context<ExposePrivacyPool>) -> AnchorResult<()> {
        msg!("IDL exposure for PrivacyPool");
        Ok(())
    }

}

/// Placeholder proof verifier
fn verify_zk_proof(_proof: &[u8], _inputs: &[u8]) -> AnchorResult<bool> {
    Ok(true)
}

// ============================================================
//                      ACCOUNTS CONTEXTS
// ============================================================
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + PrivacyPool::LEN,
        seeds = [b"privacy_pool", authority.key().as_ref()],
        bump
    )]
    pub privacy_pool: Account<'info, PrivacyPool>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + ProtocolState::LEN,
        seeds = [b"protocol_state", authority.key().as_ref()],
        bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    #[account(
        init_if_needed,
        payer = authority,
        token::mint = mint,
        token::authority = protocol_state,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ShieldTokens<'info> {
    #[account(mut)]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub privacy_pool: Account<'info, PrivacyPool>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ShieldTokens<'info> {
    pub fn transfer_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct UnshieldTokens<'info> {
    #[account(mut)]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub privacy_pool: Account<'info, PrivacyPool>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> UnshieldTokens<'info> {
    pub fn transfer_from_vault(
        &self,
    ) -> AnchorResult<CpiContext<'_, '_, '_, 'info, Transfer<'info>>> {
        let bump = self.protocol_state.state_bump;

        let proto_auth = self.protocol_state.authority;

        let bump_static: &'static [u8] = Box::leak(Box::new([bump]));
        let auth_bytes: &'static [u8] = Box::leak(Box::new(proto_auth.to_bytes()));
        let seeds_static: &'static [&'static [u8]] =
            Box::leak(Box::new([b"protocol_state", auth_bytes, bump_static]));
        let signer_seeds_static: &'static [&'static [&'static [u8]]] =
            Box::leak(Box::new([seeds_static]));

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.protocol_state.to_account_info(),
        };

        Ok(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds_static,
        ))
    }
}

#[derive(Accounts)]
pub struct PrivateTransfer<'info> {
    #[account(mut)]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub privacy_pool: Account<'info, PrivacyPool>,
}


#[derive(Accounts)]
pub struct ExposePrivacyPool<'info> {
    pub privacy_pool: Account<'info, PrivacyPool>,
}


// ============================================================
//                        ERROR CODES
// ============================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid zero-knowledge proof")]
    InvalidProof,
    #[msg("Double spend detected")]
    DoubleSpend,
    #[msg("Failed to parse Merkle tree data")]
    MerkleTreeParseError,
    #[msg("Failed to insert leaf into Merkle tree")]
    MerkleTreeInsertError,
}
