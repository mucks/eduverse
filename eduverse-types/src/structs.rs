#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;

#[derive(Clone, Default, Eq, PartialEq, Debug, Copy)]
#[cfg_attr(
    feature = "anchor_contract",
    derive(AnchorSerialize, AnchorDeserialize)
)]
pub struct ScheduleItem {
    pub teacher_id: u32,
    pub lesson_id: u32,
}
