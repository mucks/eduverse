use crate::errors;
use crate::state::{Lesson, ProfileById, Student, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;

#[event]
pub struct LessonBegins {
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
lesson_id: u32,
student_id: u32,
)]
pub struct LessonBegin<'info> {
    #[account(constraint = (payer.key() == teacher_profile.authority || payer.key() == student_profile.authority) @errors::ErrorCode::NotAuthorized)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    seeds = ["student_by_id".as_bytes(), &student_id.to_le_bytes()],
    bump
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    #[account(address = student_by_id.profile_key)]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    mut,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &lesson_id.to_le_bytes()],
    bump,
    constraint = lesson.student_id == student_id,
    constraint = lesson.status_teacher == LessonState::Approved @errors::ErrorCode::LessonStateNotApproved
    )]
    pub lesson: Box<Account<'info, Lesson>>,
}

/// Starts the lesson
pub fn handler(
    ctx: Context<LessonBegin>,
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
) -> Result<()> {
    let lesson = &mut ctx.accounts.lesson;

    // Lesson fee must have been deposited; fee_total may be 0 if teacher accepted that TODO move error into constraint
    if lesson.fee_total != lesson.fee_deposited {
        return Err(errors::ErrorCode::LessonNotFunded.into());
    }

    // Check that the start time for this lesson is right TODO decide logic; make constraint
    let current_time = Clock::get().unwrap().unix_timestamp as u64;
    if current_time < lesson.timestamp || lesson.timestamp + 3600 > current_time {
        // return Err(errors::ErrorCode::LessonScheduledAtDifferentTime.into()); TODO deactivated for testing; how to advance blockchain in anchor?
    }

    // Update the lesson state based on who submitted the tx
    if *ctx.accounts.payer.key == ctx.accounts.teacher_profile.authority {
        lesson.status_teacher = LessonState::Started
    } else if *ctx.accounts.payer.key == ctx.accounts.student_profile.authority {
        lesson.status_student = LessonState::Started
    };

    emit!(LessonBegins {
        teacher_id,
        lesson_id,
        student_id
    });

    Ok(())
}
