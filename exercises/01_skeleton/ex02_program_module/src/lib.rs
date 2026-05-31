use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

// TODO: `#[program]` must be applied to a `mod`, not a function. Anchor
// builds the on-chain dispatcher from every `pub fn` *inside* that module.
//
// Wrap `say_hi` in a module so it ends up looking like:
//
//     #[program]
//     pub mod ex02_program_module {
//         use super::*;
//         pub fn say_hi(_ctx: Context<Greet>) -> Result<()> { Ok(()) }
//     }
//
// The `use super::*;` line matters: handler signatures reference items
// defined at the crate root (like `Greet`), and without it those names
// won't resolve inside the module.
#[program]
pub fn say_hi(_ctx: Context<Greet>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Greet {}
