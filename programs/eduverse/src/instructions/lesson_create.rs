use crate::errors;
use crate::state::{Lesson, ProfileById, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct LessonCreated {
    teacher_id: u32,
    student_id: u32,
    lesson_id: u32,
    subject_id: u32,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
student_id: u32,
subject_id: u32,
fee: u64,
duration: u16,
date_time: u64,
)]
pub struct LessonCreate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, //TODO either teacher or student; dont have the teacher key; does it matter if creator pays for it

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump,
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(
    mut,
    seeds = ["teacher".as_bytes(), teacher_by_id.profile_key.as_ref()],
    bump
    )]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    init,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &teacher_profile.count_lessons.to_le_bytes()],
    bump,
    payer = payer,
    space = Lesson::LEN
    )]
    pub lesson: Box<Account<'info, Lesson>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<LessonCreate>,
    teacher_id: u32,
    student_id: u32,
    subject_id: u32,
    fee: u64,
    duration: u16,
    date_time: u64,
) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;
    let lesson_id = teacher_profile.count_lessons;

    // Attempt to register this lesson on the teachers schedule
    //TODO make DoS expensive; increase lesson account size?
    if !teacher_profile.schedule_lesson(subject_id) {
        return Err(errors::ErrorCode::ScheduleLimitReached.into());
    }

    // Increase total number of lessons created by teacher
    teacher_profile.count_lessons = teacher_profile
        .count_lessons
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store lesson related data
    let lesson = &mut ctx.accounts.lesson;
    lesson.timestamp = date_time;
    lesson.duration = duration;
    lesson.fee_total = fee;
    lesson.fee_deposited = 0;
    lesson.repeat = 0;
    lesson.cancel = 0;
    lesson.student = student_id;
    lesson.subject_id = subject_id;

    emit!(LessonCreated {
        teacher_id,
        student_id,
        lesson_id,
        subject_id,
    });

    Ok(())
}
