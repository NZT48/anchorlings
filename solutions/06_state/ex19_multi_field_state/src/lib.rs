use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex19_multi_field_state {
    use super::*;
    pub fn birthday(ctx: Context<Birthday>) -> Result<()> {
        ctx.accounts.profile.age += 1;
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
