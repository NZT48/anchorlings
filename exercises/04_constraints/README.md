# Chapter 04 — Constraints

`#[account(...)]` attributes on Accounts struct fields tell anchor to enforce
properties before your handler runs (or, in some cases, to give the field
extra capabilities the dispatcher needs to set up).

Most constraints are runtime-enforced — they show up here as test-tier
exercises because compiling alone doesn't prove they're correct. A few
(like `has_one`) are checked at macro-expansion time because they reference
fields the compiler can see.

Reference: <https://book.anchor-lang.com/anchor_in_depth/the_accounts_struct.html>
