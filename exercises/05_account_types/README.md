# Chapter 05 — Account types

`AccountInfo<'info>` is raw — anchor hands you the bytes and lets you do
whatever. The typed wrappers add validation and ergonomic access:

- `Account<'info, T>` — deserialises the account data into `T` after
  checking the discriminator anchor's `#[account]` macro emitted.
- `Signer<'info>` — checks `is_signer == true`.
- `SystemAccount<'info>` — checks the account's owner is the System Program.
- `Program<'info, T>` — checks the account key matches `T::id()`.

Reference: <https://book.anchor-lang.com/anchor_in_depth/account_types.html>
