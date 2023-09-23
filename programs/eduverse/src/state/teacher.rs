use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Teacher {
    /// Version
    pub version: u8,

    /// Profile number
    pub profile_id: u32,

    /// Profile name
    pub title: String,

    /// General availability
    pub availability: u8, //TODO

    /// Time zone
    pub timezone: u8,

    /// Contact website
    pub website: String,

    /// Contact telegram
    pub telegram: String,

    /// Contact twitter
    pub twitter: String,

    /// The number of reviews the teacher has
    pub count_reviews: u16,

    /// Total stars received (For caching - fast display of overall rating)
    pub count_stars: u32,

    /// The number of lessons the teacher created
    pub count_lessons: u32,

    /// Subjects this teacher has registered
    pub subjects_registered: [u32; 25],
}

impl Teacher {
    pub const LEN: usize = std::mem::size_of::<Teacher>() + 600; //TODO

    pub fn add_subject(&mut self, subject_id: u32) -> bool {
        for itm in &mut self.subjects_registered {
            // Not a guarantee that the subject does not appear after some "hole" (created after removing another subject)
            if *itm == subject_id {
                return false;
            } else if *itm == 0 {
                *itm = subject_id;
                return true;
            }
        }
        false
    }
}
