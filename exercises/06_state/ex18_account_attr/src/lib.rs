use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod ex18_account_attr {
    use super::*;
    pub fn read(_ctx: Context<Read>) -> Result<()> {
        Ok(())
    }
}

// TODO: `Account<'info, Profile>` needs `Profile` to implement a handful of
// anchor traits (`AccountSerialize`, `AccountDeserialize`, `Owner`, and a
// discriminator). The `#[account]` attribute generates all of them. Add it
// above the struct.
pub struct Profile {
    pub name: String,
    pub age: u8,
}

#[derive(Accounts)]
pub struct Read<'info> {
    pub profile: Account<'info, Profile>,
}
