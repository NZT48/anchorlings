use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex04_handler_arg {
    use super::*;

    // TODO: anchor handlers may take additional borsh-serialisable arguments
    // after `Context<T>`. The body below references a `count` that hasn't
    // been declared yet — add it as a `u64` parameter.
    //
    //   pub fn set_count(_ctx: Context<SetCount>, count: u64) -> Result<()>
    pub fn set_count(_ctx: Context<SetCount>) -> Result<()> {
        msg!("count = {}", count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetCount {}
