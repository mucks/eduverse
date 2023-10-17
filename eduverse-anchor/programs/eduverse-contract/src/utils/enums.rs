crate::use_anchor_encoding!();

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Eq, PartialEq, Debug, Copy)]
pub enum LessonState {
    #[default]
    Pending,
    Approved,
    Started,
    Ended,
}
