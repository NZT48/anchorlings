use anchor_lang::prelude::*;

// TODO: every Anchor program needs a unique on-chain address declared with
// `declare_id!`. Replace the placeholder below with a valid base58 program ID.
// Any valid base58-encoded 32-byte pubkey works for learning; the system
// program ID `11111111111111111111111111111112` is a fine stand-in.
declare_id!("placeholder");

#[program]
pub mod ex01_declare_id {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
