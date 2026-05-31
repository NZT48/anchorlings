use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex14_has_one_constraint {
    use super::*;
    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

// `has_one = owner` tells anchor: before the handler runs, check that
// `vault.owner == accounts.owner.key()`. For anchor to wire that up, the
// `Vault` data struct must actually have an `owner: Pubkey` field — the
// macro expands at compile time and references it by name.
//
// TODO: add `pub owner: Pubkey` to the `Vault` struct so the macro stops
// complaining that the field doesn't exist.
#[account]
pub struct Vault {
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}
