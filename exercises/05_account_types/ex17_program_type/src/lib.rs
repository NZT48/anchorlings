use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex17_program_type {
    use super::*;
    pub fn invoke_system(_ctx: Context<UseSystem>) -> Result<()> {
        Ok(())
    }
}

// TODO: this handler claims to receive *the* System Program account, but
// `AccountInfo` doesn't actually check the pubkey — a caller can hand in
// any account they like. Change the field to `Program<'info, System>`.
// `Program<'info, T>` enforces at runtime that the account's key equals
// `T::id()`, and anchor ships `System` (re-exported through `prelude`).
#[derive(Accounts)]
pub struct UseSystem<'info> {
    pub system_program: AccountInfo<'info>,
}
