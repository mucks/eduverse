use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Eq, PartialEq, Debug, Copy)]
pub enum LessonState {
    #[default]
    Pending,
    Approved,
    Started,
    Ended,
}
