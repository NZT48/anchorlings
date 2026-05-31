use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex27_system_transfer_cpi {
    use super::*;

    #[allow(unused_variables)]
    pub fn forward_lamports(ctx: Context<Forward>, amount: u64) -> Result<()> {
        // TODO: invoke the System Program's `transfer` instruction via a
        // CPI from inside this handler. Anchor provides typed helpers:
        //
        //     let cpi_accounts = system_program::Transfer {
        //         from: ctx.accounts.from.to_account_info(),
        //         to:   ctx.accounts.to.to_account_info(),
        //     };
        //     let cpi_ctx = CpiContext::new(
        //         ctx.accounts.system_program.to_account_info(),
        //         cpi_accounts,
        //     );
        //     system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Forward<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: receiver only, lamports get credited via the CPI.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
