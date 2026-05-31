# Chapter 11 — Cross-Program Invocations

Anchor wraps Solana's raw `invoke` / `invoke_signed` in `CpiContext`. You
pass it the target program account, an accounts struct the callee expects,
and (for PDA-signed calls) the signer seeds. Anchor ships typed helpers
for the System Program out of the box.

Reference: <https://book.anchor-lang.com/anchor_in_depth/cpis.html>
