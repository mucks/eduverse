use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct SubjectTeacher {
    /// The profile number of the teacher
    pub teacher_profile_id: u32,
    //TODO statistics? total lessons taught?, ...
}

impl SubjectTeacher {
    pub const LEN: usize = std::mem::size_of::<SubjectTeacher>();
}
