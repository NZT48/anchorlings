use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex17_program_type {
    use super::*;
    pub fn invoke_system(_ctx: Context<UseSystem>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseSystem<'info> {
    pub system_program: Program<'info, System>,
}
