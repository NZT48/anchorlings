use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex05_handler_multi_args {
    use super::*;

    // TODO: handlers can take any number of borsh-serialisable args after
    // `Context<T>`. The body references `name` (a `String`) and `amount`
    // (a `u64`) — declare both as parameters in that order.
    pub fn record(_ctx: Context<Record>) -> Result<()> {
        msg!("{} bought {} tokens", name, amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record {}
