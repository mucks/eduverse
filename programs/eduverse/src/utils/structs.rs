crate::use_anchor_encoding!();

#[derive(Clone, Default, Eq, PartialEq, Debug, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct ScheduleItem {
    pub teacher_id: u32,
    pub lesson_id: u32,
}
