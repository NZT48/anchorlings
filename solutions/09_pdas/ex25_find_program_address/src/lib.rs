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

#[derive(Accounts)]
pub struct ReadVault<'info> {
    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}
