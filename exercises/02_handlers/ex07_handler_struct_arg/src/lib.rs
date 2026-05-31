use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex07_handler_struct_arg {
    use super::*;

    pub fn configure(_ctx: Context<Configure>, cfg: Config) -> Result<()> {
        msg!("seats={} max_bid={}", cfg.seats, cfg.max_bid);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Configure {}

// TODO: every custom struct passed as a handler argument must be borsh-
// serialisable so anchor can decode it from the instruction data and
// borsh-encodable so callers can build it. Add the right derives:
//
//   #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
//
// (Anchor re-exports `AnchorSerialize` / `AnchorDeserialize` from `prelude`.)
pub struct Config {
    pub seats: u32,
    pub max_bid: u64,
}
