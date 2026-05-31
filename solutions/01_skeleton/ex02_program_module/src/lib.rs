use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex02_program_module {
    use super::*;
    pub fn say_hi(_ctx: Context<Greet>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Greet {}
