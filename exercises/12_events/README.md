# Chapter 12 — Events

`#[event]` on a struct declares a borsh-encodable event type. `emit!(...)`
writes the discriminator-prefixed payload into the transaction logs via
`sol_log_data`. Off-chain consumers (RPC subscribers, anchor's JS client,
your tests) can scan for it.

Reference: <https://book.anchor-lang.com/anchor_in_depth/events.html>
