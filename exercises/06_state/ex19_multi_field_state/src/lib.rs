use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex19_multi_field_state {
    use super::*;
    pub fn birthday(_ctx: Context<Birthday>) -> Result<()> {
        // TODO: bump `profile.age` by 1. Anchor reads the whole `Profile`
        // out of the account at handler entry and writes it back at exit,
        // so a field-level mutation is enough — `balance` will be left
        // untouched automatically.
        Ok(())
    }
}

#[account]
pub struct Profile {
    pub age: u8,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Birthday<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
}
