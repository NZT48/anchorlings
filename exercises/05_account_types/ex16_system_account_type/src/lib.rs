use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex16_system_account_type {
    use super::*;
    pub fn check(_ctx: Context<Check>) -> Result<()> {
        Ok(())
    }
}

// TODO: this handler is supposed to accept only accounts owned by the
// System Program (i.e. ordinary wallets / unused PDAs). `AccountInfo`
// will accept anything. Change the field to `SystemAccount<'info>` and
// anchor's dispatcher will verify `info.owner == system_program::ID` for
// you at runtime.
#[derive(Accounts)]
pub struct Check<'info> {
    pub system_owned: AccountInfo<'info>,
}
