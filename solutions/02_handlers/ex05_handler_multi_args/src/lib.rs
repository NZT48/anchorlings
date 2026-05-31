use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex05_handler_multi_args {
    use super::*;
    pub fn record(_ctx: Context<Record>, name: String, amount: u64) -> Result<()> {
        msg!("{} bought {} tokens", name, amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record {}
