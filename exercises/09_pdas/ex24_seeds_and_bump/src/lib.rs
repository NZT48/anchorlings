use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex24_seeds_and_bump {
    use super::*;
    pub fn touch(_ctx: Context<TouchVault>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub balance: u64,
}

// TODO: declaring `seeds = [...]` on its own isn't enough — anchor needs
// to know which bump produced the address, otherwise it can't re-derive
// and compare. Add `bump` next to the seeds:
//
//     #[account(seeds = [b"vault", user.key().as_ref()], bump)]
//
// (No value after `bump` tells anchor to find and use the canonical bump.)
#[derive(Accounts)]
pub struct TouchVault<'info> {
    #[account(seeds = [b"vault", user.key().as_ref()])]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}
