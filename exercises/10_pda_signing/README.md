# Chapter 10 — PDA signing

A PDA has no private key, so the program that controls its seeds is the
only authority that can sign for it. Anchor remembers the bump it found
when validating the PDA so you can build the signer-seeds slice that
proves ownership during CPIs.

Reference: <https://book.anchor-lang.com/anchor_in_depth/pdas.html#cross-program-invocations-with-pdas>
