//! Tiny shim for anchorlings test-tier exercises.
//!
//! `solana-program-test`'s `processor!` macro wants
//! `fn(&Pubkey, &[AccountInfo<'_>], &[u8]) -> ProgramResult` — a fn pointer
//! with independent lifetimes on the slice vs. its `AccountInfo` elements.
//!
//! Anchor's generated `entry` ties them together (`&'info [AccountInfo<'info>]`)
//! because the `Accounts` derive needs the accounts to outlive the call.
//!
//! `anchor_entry!(name, my_program)` emits a free function `name` with the
//! signature solana-program-test wants; it forwards into `my_program::entry`
//! after laundering the lifetimes with a transmute. At call time the slice
//! and its elements all live for the same duration, so this is sound.
//!
//! Usage:
//! ```ignore
//! anchorlings_test_kit::anchor_entry!(entry, my_program);
//!
//! #[tokio::test]
//! async fn it_runs() {
//!     let (mut banks, payer, blockhash) =
//!         ProgramTest::new("my_program", my_program::ID, processor!(entry))
//!             .start().await;
//! }
//! ```

pub use anchor_lang::solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

#[macro_export]
macro_rules! anchor_entry {
    ($name:ident, $program_crate:ident) => {
        fn $name(
            program_id: &$crate::Pubkey,
            accounts: &[$crate::AccountInfo],
            data: &[u8],
        ) -> $crate::ProgramResult {
            let accounts: &[$crate::AccountInfo] =
                unsafe { ::core::mem::transmute(accounts) };
            $program_crate::entry(program_id, accounts, data)
        }
    };
}
