# Chapter 03 — Accounts structs

`Context<T>` carries the accounts a handler receives. `T` must be a struct
that derives `Accounts`, declares the `'info` lifetime, and lists each
account the handler needs as a field.

Reference: <https://book.anchor-lang.com/anchor_in_depth/accounts.html>
