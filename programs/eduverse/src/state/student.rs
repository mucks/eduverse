use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Student {
    /// Version
    pub version: u8,

    /// Profile number
    pub profile_id: u32,

    /// Authority
    pub authority: Pubkey,

    /// Profile name
    pub title: String,

    /// Contact details
    pub contact_info: String,
}

impl Student {
    pub const LEN: usize = std::mem::size_of::<Student>() + 600; //TODO
}
