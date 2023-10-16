crate::prelude!();

#[account]
#[derive(Default)]
pub struct ReviewById {
    /// Profile key
    pub profile_key: Pubkey,
}

impl ReviewById {
    pub const LEN: usize = std::mem::size_of::<ReviewById>() + 8;
}
