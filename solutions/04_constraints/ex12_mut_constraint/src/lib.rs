use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex12_mut_constraint {
    use super::*;
    pub fn set_value(ctx: Context<SetValue>, value: u64) -> Result<()> {
        ctx.accounts.counter.value = value;
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
