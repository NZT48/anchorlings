use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex15_account_type {
    use super::*;
    pub fn show_value(ctx: Context<Show>) -> Result<()> {
        msg!("counter = {}", ctx.accounts.counter.value);
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Show<'info> {
    pub counter: Account<'info, Counter>,
}
