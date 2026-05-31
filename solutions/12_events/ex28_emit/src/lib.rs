use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex28_emit {
    use super::*;

    pub fn announce(_ctx: Context<Announce>, message: String) -> Result<()> {
        emit!(Greeted { message });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Announce {}

#[event]
pub struct Greeted {
    pub message: String,
}
