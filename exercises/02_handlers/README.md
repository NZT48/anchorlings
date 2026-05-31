# Chapter 02 — Handlers

The shape of an instruction handler: it lives inside `#[program]`, takes
`Context<T>` as its first parameter (always), may take additional
borsh-serialisable arguments after that, and returns `Result<T>` (most
often `Result<()>`).

Reference: <https://book.anchor-lang.com/anchor_in_depth/instructions.html>
