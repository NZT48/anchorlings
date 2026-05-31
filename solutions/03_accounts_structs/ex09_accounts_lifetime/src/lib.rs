use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex09_accounts_lifetime {
    use super::*;
    pub fn greet(_ctx: Context<Greet>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Greet<'info> {
    pub greeter: AccountInfo<'info>,
}
