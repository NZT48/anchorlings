# Chapter 07 — Init & space

`#[account(init, payer = ..., space = ...)]` is anchor's shortcut for
"create this account in the system program, sized for my struct, and pay
the rent from this signer". The compiler will tell you when you forget a
required field — these exercises walk through the common ones.

Reference: <https://book.anchor-lang.com/anchor_in_depth/account_constraints.html#initialization>
