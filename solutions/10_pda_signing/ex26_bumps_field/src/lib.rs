use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex26_bumps_field {
    use super::*;
    pub fn show_bump(ctx: Context<ShowBump>) -> Result<()> {
        let bump: u8 = ctx.bumps.vault;
        msg!("vault bump = {}", bump);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ShowBump<'info> {
    /// CHECK: PDA validated by anchor via the `seeds` + `bump` constraint.
    #[account(seeds = [b"vault"], bump)]
    pub vault: UncheckedAccount<'info>,
}
