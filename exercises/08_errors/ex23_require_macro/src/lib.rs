use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex23_require_macro {
    use super::*;
    #[allow(unused_variables)]
    pub fn min_age(_ctx: Context<MinAge>, age: u8) -> Result<()> {
        // TODO: reject the call with `MyError::TooYoung` when `age` is
        // less than 18. The `require!` macro is the ergonomic form:
        //
        //     require!(age >= 18, MyError::TooYoung);
        //
        // When the condition is false, anchor returns the named error and
        // the transaction aborts before any state changes.
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
