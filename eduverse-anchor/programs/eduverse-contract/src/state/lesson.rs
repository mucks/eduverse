use crate::utils::LessonState;
crate::prelude!();

#[account]
#[derive(Debug, Default)]
pub struct Lesson {
    /// Version
    pub version: u8,

    /// Who is the teacher
    pub teacher_id: u32,

    /// Who is the student
    pub student_id: u32,

    /// What is the subject being taught
    pub subject_id: u32,

    /// The teachers response to this lesson
    pub status_teacher: LessonState,

    /// The students response to this lesson
    pub status_student: LessonState,

    /// When this lesson is due to take place
    pub timestamp: u64,

    /// Length of lesson in minutes
    pub duration: u16,

    /// Public key of a SPL token mint or None for SOL
    pub token_address: Option<Pubkey>,

    /// Total price for the lesson
    pub fee_total: u64,

    /// Amount of funds deposited
    pub fee_deposited: u64,

    /// Is this a repeating lesson?
    pub repeat: u8, //TODO logic with payment etc.

    /// Up till when can this lesson be cancelled?
    pub cancel: u8, //TODO may need a deposit deadline, cancel details, refunds? .. or just request to cancel + refund
}

impl Lesson {
    pub const LEN: usize = std::mem::size_of::<Lesson>() + 400; // likely needs future edits
}
