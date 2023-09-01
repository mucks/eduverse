use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Teacher {
    /// Version
    pub version: u8,

    /// Profile name
    pub title: String,

    /// Contact website
    pub website: String,

    /// Contact telegram
    pub telegram: String,

    /// Contact twitter
    pub twitter: String,

    /// The number of reviews the teacher has
    pub count_reviews: u16,

    /// Total stars received (For caching - fast display of overall rating)
    pub count_stars: u32,
}

impl Teacher {
    pub const LEN: usize = std::mem::size_of::<Teacher>() + 600; //TODO
}
