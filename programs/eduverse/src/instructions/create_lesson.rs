use crate::errors;
use crate::state::{Lesson, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct LessonCreated {
    teacher: Pubkey,
    subject: u32,
    student: u32,
}

#[derive(Accounts)]
#[instruction(
student: u32,
subject: u32,
fee: u64,
duration: u16,
date_time: u64,
)]
pub struct CreateLesson<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher".as_bytes(), payer.key().as_ref()],
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
    ctx: Context<CreateLesson>,
    student: u32,
    subject: u32,
    fee: u64,
    duration: u16,
    date_time: u64,
) -> Result<()> {
    // Store data
    let lesson = &mut ctx.accounts.lesson;
    lesson.timestamp = date_time;
    lesson.duration = duration;
    lesson.fee_total = fee;
    lesson.fee_deposited = 0;
    lesson.repeat = 0;
    lesson.cancel = 0;
    lesson.student = student;
    lesson.subject_id = subject;

    // Increase total number of lessons created by teacher
    let teacher = &mut ctx.accounts.teacher_profile;
    teacher.count_lessons = teacher
        .count_lessons
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(LessonCreated {
        teacher: ctx.accounts.payer.key(),
        subject,
        student
    });

    Ok(())
}
