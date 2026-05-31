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

// TODO: `emit!` writes an event's discriminator + borsh-encoded body via
// `sol_log_data`, where off-chain consumers can pick it up. For that to
// compile, `Greeted` needs the discriminator constant, an `Event` impl,
// and the borsh derives — all of which `#[event]` generates. Add it.
pub struct Greeted {
    pub message: String,
}
