use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Eq, PartialEq, Debug, Copy)]
pub struct ScheduleItem {
    pub teacher_id: u32,
    pub lesson_id: u32,
}
