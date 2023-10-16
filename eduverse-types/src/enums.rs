#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;

#[derive(Clone, Default, Eq, PartialEq, Debug, Copy)]
#[cfg_attr(
    feature = "anchor_contract",
    derive(AnchorSerialize, AnchorDeserialize)
)]
pub enum LessonState {
    #[default]
    Pending,
    Approved,
    Started,
    Ended,
}
