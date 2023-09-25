use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow Error")]
    OverflowError, // 6000

    #[msg("Not authorized to perform this action")]
    NotAuthorized, // 6001

    #[msg("You can not add any more subjects")]
    SubjectLimitReached, // 6002

    #[msg("This teachers schedule is currently full")]
    ScheduleLimitReached, // 6003
}
