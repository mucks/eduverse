crate::prelude!();

#[account]
#[derive(Debug, Default)]
pub struct SubjectTeacher {
    /// The profile number of the teacher
    pub teacher_profile_id: u32,
    //TODO statistics? total number of lessons taught?, ...
}

impl SubjectTeacher {
    pub const LEN: usize = std::mem::size_of::<SubjectTeacher>() + 8;
}
