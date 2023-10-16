crate::prelude!();

#[account]
#[cfg_attr(
    all(not(feature = "anchor"), feature = "wasm"),
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Default, Debug)]
pub struct Config {
    /// Version
    pub version: u8,

    /// The number of teacher profiles
    pub count_teachers: u32,

    /// The number of student profiles
    pub count_students: u32,
}

impl Config {
    pub const LEN: usize = std::mem::size_of::<Config>() + 600;
}
