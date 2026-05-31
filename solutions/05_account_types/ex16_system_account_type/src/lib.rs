use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex16_system_account_type {
    use super::*;
    pub fn check(_ctx: Context<Check>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Check<'info> {
    pub system_owned: SystemAccount<'info>,
}
