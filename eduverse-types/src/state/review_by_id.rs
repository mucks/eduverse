#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[cfg_attr(feature = "anchor_contract", account)]
#[derive(Default)]
pub struct ReviewById {
    /// Profile key
    pub profile_key: Pubkey,
}

impl ReviewById {
    pub const LEN: usize = std::mem::size_of::<ReviewById>() + 8;
}
