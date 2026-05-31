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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Config {
    pub seats: u32,
    pub max_bid: u64,
}
