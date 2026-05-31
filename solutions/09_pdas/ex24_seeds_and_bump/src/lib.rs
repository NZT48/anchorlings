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

#[derive(Accounts)]
pub struct TouchVault<'info> {
    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}
