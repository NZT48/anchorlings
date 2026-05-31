use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex11_distinct_accounts {
    use super::*;
    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        Ok(())
    }
    pub fn pong(_ctx: Context<Pong>) -> Result<()> {
        Ok(())
    }
}

// Each handler picks its own `Context<T>` type, and each `T` is a separate
// Accounts struct — handlers don't have to share account layouts.

#[derive(Accounts)]
pub struct Ping {}

// TODO: define a second `Accounts` struct named `Pong` so the `pong`
// handler's `Context<Pong>` resolves. It can be empty for now.
