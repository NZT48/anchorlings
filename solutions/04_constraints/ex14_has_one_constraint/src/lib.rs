use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex14_has_one_constraint {
    use super::*;
    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}
