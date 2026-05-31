use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex04_handler_arg {
    use super::*;
    pub fn set_count(_ctx: Context<SetCount>, count: u64) -> Result<()> {
        msg!("count = {}", count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetCount {}
