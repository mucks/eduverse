use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TeacherSubject {
    /// The subject id being taught
    pub subject_id: u32
}

impl TeacherSubject {
    pub const LEN: usize = std::mem::size_of::<TeacherSubject>();
}
