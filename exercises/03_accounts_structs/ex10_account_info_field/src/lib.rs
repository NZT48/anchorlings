use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex10_account_info_field {
    use super::*;
    pub fn greet(ctx: Context<Greet>) -> Result<()> {
        msg!(
            "greeter {} waves at recipient {}",
            ctx.accounts.greeter.key,
            ctx.accounts.recipient.key,
        );
        Ok(())
    }
}

// TODO: the handler reads two accounts via `ctx.accounts.*`, but only
// `greeter` is declared. Add `recipient` as a second `AccountInfo<'info>`
// field. Any account a handler touches must be declared here so anchor
// includes it in the dispatcher and validates it.
#[derive(Accounts)]
pub struct Greet<'info> {
    pub greeter: AccountInfo<'info>,
}
