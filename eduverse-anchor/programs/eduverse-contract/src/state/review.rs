crate::prelude!();

#[account]
#[derive(Debug, Default)]
pub struct Review {
    /// Version
    pub version: u8,

    /// How many stars were given
    pub stars: u8,

    /// Date of review TODO fairly optional account creation date is there as well
    pub created_at: u64,

    /// Content of the review
    pub text: String,
}

impl Review {
    pub const LEN: usize = std::mem::size_of::<Review>() + 600; //TODO
}
