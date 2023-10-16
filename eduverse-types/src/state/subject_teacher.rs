#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;

#[cfg_attr(feature = "anchor_contract", account)]
#[derive(Default)]
pub struct SubjectTeacher {
    /// The profile number of the teacher
    pub teacher_profile_id: u32,
    //TODO statistics? total number of lessons taught?, ...
}

impl SubjectTeacher {
    pub const LEN: usize = std::mem::size_of::<SubjectTeacher>() + 8;
}
