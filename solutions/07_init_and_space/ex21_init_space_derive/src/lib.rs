use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex21_init_space_derive {
    use super::*;
    pub fn create(_ctx: Context<CreateProfile>) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
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
