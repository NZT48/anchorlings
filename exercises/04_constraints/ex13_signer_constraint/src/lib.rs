use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex13_signer_constraint {
    use super::*;
    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

// TODO: this `withdraw` is meant to be authorised by `owner`, but the
// struct accepts any `AccountInfo` — nothing forces the caller to actually
// have signed for that account. Add the `signer` constraint:
//
//     #[account(signer)]
//     pub owner: AccountInfo<'info>,
//
// With it, anchor's dispatcher rejects the instruction at runtime if the
// account meta for `owner` doesn't have `is_signer = true`.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub owner: AccountInfo<'info>,
}
