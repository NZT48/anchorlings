use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex06_handler_return_value {
    use super::*;
    pub fn answer(_ctx: Context<Answer>) -> Result<u64> {
        Ok(42)
    }
}

#[derive(Accounts)]
pub struct Answer {}
