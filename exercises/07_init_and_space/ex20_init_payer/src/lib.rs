use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex20_init_payer {
    use super::*;
    pub fn create(_ctx: Context<CreateCounter>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

// TODO: `init` allocates the account and pays its rent from the program-
// supplied `payer` field. Anchor requires you to name which signer it
// should charge — extend the constraint to:
//
//     #[account(init, payer = payer, space = 8 + 8)]
//
// The `8 + 8` is anchor's 8-byte discriminator plus 8 bytes for `u64`.
#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(init, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
