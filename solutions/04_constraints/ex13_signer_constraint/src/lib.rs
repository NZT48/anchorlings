use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex13_signer_constraint {
    use super::*;
    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}
