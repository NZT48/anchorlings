use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex23_require_macro {
    use super::*;
    pub fn min_age(_ctx: Context<MinAge>, age: u8) -> Result<()> {
        require!(age >= 18, MyError::TooYoung);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MinAge {}

#[error_code]
pub enum MyError {
    #[msg("must be at least 18")]
    TooYoung,
}
