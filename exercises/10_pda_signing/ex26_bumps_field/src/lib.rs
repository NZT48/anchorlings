use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex26_bumps_field {
    use super::*;
    pub fn show_bump(ctx: Context<ShowBump>) -> Result<()> {
        // TODO: anchor records the canonical bump it used to derive each
        // PDA-validated account in `Context::bumps`. The field is a struct
        // (one named field per PDA account), so a plain `ctx.bumps` is
        // the *whole struct* — not a `u8`. Read the vault's bump out via
        //
        //     let bump: u8 = ctx.bumps.vault;
        //
        // This is the bump you'd then pass to `invoke_signed` as part of
        // your signer seeds.
        let bump: u8 = ctx.bumps;
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
