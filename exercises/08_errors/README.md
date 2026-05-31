# Chapter 08 — Errors

`#[error_code]` on an enum gives each variant a stable numeric code, a
`#[msg("...")]`-derived human message, and the `From` impls anchor needs
so you can return a variant straight from `require!` or `err!`.

Reference: <https://book.anchor-lang.com/anchor_in_depth/errors.html>
