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

// TODO: `AccountInfo` is just a raw byte slice — it doesn't know that the
// account stores a `Counter`, so `ctx.accounts.counter.value` doesn't
// compile. Change the field to `Account<'info, Counter>` so anchor
// deserialises the data for you (and checks the discriminator while it's
// at it).
#[derive(Accounts)]
pub struct Show<'info> {
    pub counter: AccountInfo<'info>,
}
