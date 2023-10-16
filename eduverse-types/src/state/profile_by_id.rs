#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[cfg_attr(feature = "anchor_contract", account)]
#[derive(Default)]
pub struct ProfileById {
    /// Profile key
    pub profile_key: Pubkey,
}

impl ProfileById {
    pub const LEN: usize = std::mem::size_of::<ProfileById>() + 8;
}
