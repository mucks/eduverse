crate::prelude!();

#[account]
#[derive(Debug, PartialEq)]
pub struct Teacher {
    /// Version
    pub version: u8,

    /// Profile number
    pub profile_id: u32,

    /// Authority
    pub authority: Pubkey,

    /// The date that this teacher registered
    pub registered_at: i64,

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

    /// The number of lessons cancelled
    pub count_lessons_cancelled: u32,

    /// Currently relevant lessons
    pub lesson_data: [u32; 200],

    /// Subjects this teacher has registered
    pub subjects_registered: [u32; 25],
}

impl Teacher {
    pub const LEN: usize = std::mem::size_of::<Teacher>() + 600; //TODO

    /// Attempts to register a new subject on this teacher. Can fail if the limit is reached
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

    /// Attempts to schedule a new lesson with this teacher. Can fail if the limit is reached
    pub fn schedule_add(&mut self, lesson_id: u32) -> bool {
        for itm in &mut self.lesson_data {
            if *itm == 0 {
                *itm = lesson_id;
                return true;
            }
        }
        false
    }

    /// Remove a lesson from this teacher
    pub fn schedule_remove(&mut self, lesson_id: u32) {
        for itm in &mut self.lesson_data {
            if *itm == lesson_id {
                *itm = 0;
                return;
            }
        }
    }

    /// Attempts to find a specific subject in the list of subjects taught by this teacher
    pub fn teaches_subject(&self, subject_id: u32) -> bool {
        self.subjects_registered.contains(&subject_id)
    }
}

impl Default for Teacher {
    fn default() -> Self {
        Self {
            version: 0,
            profile_id: 0,
            authority: Default::default(),
            registered_at: 0,
            title: Default::default(),
            availability: 0,
            timezone: 0,
            website: Default::default(),
            telegram: Default::default(),
            twitter: Default::default(),
            count_reviews: 0,
            count_stars: 0,
            // Lesson 0 won't be a valid id, since lesson references are stored in an array where 0 means empty
            count_lessons: 1,
            count_lessons_cancelled: 0,
            lesson_data: [0u32; 200],
            subjects_registered: Default::default(),
        }
    }
}
