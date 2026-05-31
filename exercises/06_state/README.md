# Chapter 06 — State

`#[account]` on a struct turns it into an on-chain state type. The macro
emits the discriminator, the `AccountSerialize` / `AccountDeserialize`
impls, and the `Owner` impl pointing at the current crate's program ID.

Reference: <https://book.anchor-lang.com/anchor_in_depth/accounts.html#account>
