use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex22_error_code {
    use super::*;
    pub fn check_positive(_ctx: Context<CheckPositive>, x: i64) -> Result<()> {
        require!(x > 0, MyError::Negative);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckPositive {}

// TODO: define `MyError` so the `require!` line above resolves. Anchor's
// `#[error_code]` attribute turns each variant of an enum into a typed
// program error with a stable numeric code and a human-readable message.
// Define:
//
//     #[error_code]
//     pub enum MyError {
//         #[msg("number must be positive")]
//         Negative,
//     }
