use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Lesson {
    /// Version
    pub version: u8,

    /// When this lesson is due to take place
    pub timestamp: u64,

    /// Length of lesson in minutes
    pub duration: u16,

    /// Total price for the lesson
    pub fee_total: u64,

    /// Amount of funds deposited
    pub fee_deposited: u64,

    /// Is this a repeating lesson?
    pub repeat: u8, //TODO logic with payment etc.

    /// Up till when can this lesson be cancelled?
    pub cancel: u8, //TODO may need a deposit deadline, cancel details, refunds? .. or just request to cancel + refund

    /// Who is the student
    pub student: u32, //TODO profile u32 or pubkey? (depends on whether students should havea profile or not)

    /// What is the subject being taught
    pub subject_id: u32, //TODO make this an enum or leave it as a number for the frontend
}

impl Lesson {
    pub const LEN: usize = std::mem::size_of::<Lesson>();
}
