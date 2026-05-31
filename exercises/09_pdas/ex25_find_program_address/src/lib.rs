use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex25_find_program_address {
    use super::*;
    pub fn read_vault(_ctx: Context<ReadVault>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub balance: u64,
}

// TODO: every user should get their own vault. Scope the PDA to the user
// by adding `user.key().as_ref()` to the seeds so different users derive
// different vaults:
//
//     seeds = [b"vault", user.key().as_ref()], bump
#[derive(Accounts)]
pub struct ReadVault<'info> {
    #[account(seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}
