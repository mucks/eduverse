crate::prelude!();
use crate::utils::ScheduleItem;

#[account]
#[derive(Debug)]
pub struct Student {
    /// Version
    pub version: u8,

    /// Profile number
    pub profile_id: u32,

    /// Authority
    pub authority: Pubkey,

    /// Profile name
    pub title: String,

    /// Contact details
    pub contact_info: String,

    /// Currently relevant lessons
    pub lesson_data: [ScheduleItem; 50],

    /// Which teachers this student can review
    pub review_data: [u32; 25],

    /// Which index of the review_data array to write next
    pub review_offset: u8,
}

impl Student {
    pub const LEN: usize = std::mem::size_of::<Student>() + 600; //TODO

    /// Add the given teacher_id as reviewable by this student
    pub fn reviewable_teacher_add(&mut self, teacher_id: u32) {
        // Do not add again
        if self.review_data.contains(&teacher_id) {
            return;
        }

        // Write teacher_id to offset and increment
        self.review_data[self.review_offset as usize] = teacher_id;
        self.review_offset += 1;

        // Wrap the offset if required
        if self.review_offset == self.review_data.len() as u8 {
            self.review_offset = 0;
        }
    }

    /// Remove some teacher_id from the array of reviewable teachers
    pub fn reviewable_teacher_remove(&mut self, teacher_id: u32) -> bool {
        for itm in &mut self.review_data {
            if *itm == teacher_id {
                *itm = 0;
                return true;
            }
        }
        false
    }

    /// Attempts to schedule a new lesson with this student for the given teacher. Can fail if the limit is reached
    pub fn schedule_add(&mut self, teacher_id: u32, lesson_id: u32) -> bool {
        for itm in &mut self.lesson_data {
            if *itm == Default::default() {
                *itm = ScheduleItem {
                    teacher_id,
                    lesson_id,
                };
                return true;
            }
        }
        false
    }

    /// Remove a lesson from this student
    pub fn schedule_remove(&mut self, teacher_id: u32, lesson_id: u32) {
        for itm in &mut self.lesson_data {
            if *itm
                == (ScheduleItem {
                    teacher_id,
                    lesson_id,
                })
            {
                *itm = Default::default();
                return;
            }
        }
    }
}

impl Default for Student {
    fn default() -> Self {
        Self {
            version: 0,
            profile_id: 0,
            authority: Default::default(),
            title: Default::default(),
            contact_info: Default::default(),
            lesson_data: [Default::default(); 50],
            review_data: [0; 25],
            review_offset: 0,
        }
    }
}
