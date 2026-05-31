use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex08_accounts_derive {
    use super::*;
    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        Ok(())
    }
}

// TODO: every type used inside `Context<T>` must implement the `Accounts`
// trait. The trait is generated for you by the `#[derive(Accounts)]`
// attribute — add it to the struct below.
pub struct Ping {}
