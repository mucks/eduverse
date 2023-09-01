use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Subject {
    /// The number of teachers teaching this subject
    pub count_teachers: u32,
    //TODO statistics? total lessons taught?, ...
}

impl Subject {
    pub const LEN: usize = std::mem::size_of::<Subject>();
}
