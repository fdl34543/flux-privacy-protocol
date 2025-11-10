use anchor_lang::prelude::*;

/// Dummy Merkle tree (placeholder)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DummyMerkleTree {
    pub leaves: Vec<[u8; 32]>,
}

impl DummyMerkleTree {
    pub fn new() -> Self {
        Self { leaves: Vec::new() }
    }

    pub fn append(&mut self, leaf: [u8; 32]) -> Result<()> {
        self.leaves.push(leaf);
        Ok(())
    }
}

/// Privacy pool account (for shielded assets)
#[account]
#[derive(Default)]
pub struct PrivacyPool {
    pub tree: DummyMerkleTree,      
    pub nullifiers: Vec<[u8; 32]>,   
    pub user_balances: Vec<(Pubkey, u64)>,
    pub commitment_count: u64,
}

impl PrivacyPool {
    pub const LEN: usize = 8   // discriminator
        + (32 * 128)           // merkle leaves (lebih kecil)
        + (32 * 128)           // nullifiers
        + (40 * 50);           // user_balances

    // pub const LEN: usize = 8 + (8 * 1024) + 32 * 1024 + (64 * 100) + 8;

    pub fn is_nullifier_used(&self, n: &[u8; 32]) -> bool {
        self.nullifiers.contains(n)
    }
}
