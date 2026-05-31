use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex21_init_space_derive {
    use super::*;
    pub fn create(_ctx: Context<CreateProfile>) -> Result<()> {
        Ok(())
    }
}

// TODO: counting bytes by hand is brittle — change `space = 8 + 16` for
// every field tweak gets old fast. Anchor's `#[derive(InitSpace)]` walks
// the fields at macro-expansion time and emits an `INIT_SPACE` constant
// you can drop into your `space = ...` constraint.
//
// Add `#[derive(InitSpace)]` to `Profile` so the existing
// `space = 8 + Profile::INIT_SPACE` resolves.
#[account]
pub struct Profile {
    pub age: u8,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = payer, space = 8 + Profile::INIT_SPACE)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
