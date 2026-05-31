use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex06_handler_return_value {
    use super::*;

    // TODO: handlers don't have to return `Result<()>` — anchor will serialize
    // any borsh-encodable success value into the transaction's return data.
    // This handler is declared `Result<u64>` but the body still returns `()`.
    // Make it return the answer to life, the universe, and everything: 42.
    pub fn answer(_ctx: Context<Answer>) -> Result<u64> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Answer {}
