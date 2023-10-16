crate::prelude!();

#[account]
#[derive(Debug, Default)]
pub struct SubjectConfig {
    /// The number of teachers teaching this subject
    pub count_teachers: u32,
    //TODO statistics? total lessons taught?, ...
}

impl SubjectConfig {
    pub const LEN: usize = std::mem::size_of::<SubjectConfig>() + 8;
}
