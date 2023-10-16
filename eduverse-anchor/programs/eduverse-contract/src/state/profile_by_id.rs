crate::prelude!();

#[account]
#[derive(Debug, Default)]
pub struct ProfileById {
    /// Profile key
    pub profile_key: Pubkey,
}

impl ProfileById {
    pub const LEN: usize = std::mem::size_of::<ProfileById>() + 8;
}
