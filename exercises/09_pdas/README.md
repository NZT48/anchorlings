# Chapter 09 — PDAs

A Program-Derived Address is a `Pubkey` for which no private key exists —
solana generates it by hashing your seeds plus a found "bump" until the
result falls off the ed25519 curve. The program that owns the seeds is the
only one allowed to sign for it. Anchor builds the derivation into the
Accounts struct via `seeds = [...]` and `bump`.

Reference: <https://book.anchor-lang.com/anchor_in_depth/pdas.html>
