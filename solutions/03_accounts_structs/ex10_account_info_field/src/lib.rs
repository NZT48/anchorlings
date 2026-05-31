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

#[derive(Accounts)]
pub struct Greet<'info> {
    pub greeter: AccountInfo<'info>,
    pub recipient: AccountInfo<'info>,
}
