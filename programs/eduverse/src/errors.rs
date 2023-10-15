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
    ScheduleLimitReachedTeacher, // 6003

    #[msg("This students schedule is currently full")]
    ScheduleLimitReachedStudent, // 6004

    #[msg("This teacher does not teach the specified subject")]
    SubjectNotTaught, // 6005

    #[msg("Can not schedule lesson for right away or too far into the future")]
    LessonScheduleBadDateTime, // 6006

    #[msg("Can only approve lessons in pending state")]
    LessonStateNotPending, // 6007

    #[msg("This lesson has not yet been approved")]
    LessonStateNotApproved, // 6008

    #[msg("This lesson has not yet been funded")]
    LessonNotFunded, // 6009

    #[msg("This lesson has been scheduled to start at another time")]
    LessonScheduledAtDifferentTime, // 6010

    #[msg("This lesson has not yet been started")]
    LessonStateNotStarted, // 6011
}
