use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Student {
    /// Version
    pub version: u8,

    /// Profile name
    pub title: String,
} //TODO not sure IF needed or whats needed

impl Student {
    pub const LEN: usize = std::mem::size_of::<Student>() + 600; //TODO
}
