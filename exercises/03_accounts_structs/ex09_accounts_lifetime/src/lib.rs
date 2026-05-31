use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex09_accounts_lifetime {
    use super::*;
    pub fn greet(_ctx: Context<Greet>) -> Result<()> {
        Ok(())
    }
}

// TODO: account fields like `AccountInfo<'info>` need a lifetime that
// outlives the transaction. Anchor's convention is the named lifetime
// `'info` — declare it on the struct so the field's `'info` resolves:
//
//     pub struct Greet<'info> { ... }
#[derive(Accounts)]
pub struct Greet {
    pub greeter: AccountInfo<'info>,
}
