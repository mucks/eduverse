use crate::errors;
use crate::state::{Lesson, ProfileById, Student, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;

#[event]
pub struct LessonEnded {
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
pub struct LessonEnd<'info> {
    #[account(constraint = (payer.key() == teacher_profile.authority || payer.key() == student_profile.authority))]
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
    constraint = lesson.status_teacher == LessonState::Started @errors::ErrorCode::LessonStateNotStarted
    )]
    pub lesson: Box<Account<'info, Lesson>>,
}

pub fn handler(
    ctx: Context<LessonEnd>,
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
) -> Result<()> {
    let lesson = &mut ctx.accounts.lesson;

    // Update the lesson state based on who submitted the tx
    //TODO Do we need this action to be performed by the student?
    if *ctx.accounts.payer.key == ctx.accounts.teacher_profile.authority {
        lesson.status_teacher = LessonState::Ended
    } else if *ctx.accounts.payer.key == ctx.accounts.student_profile.authority {
        lesson.status_student = LessonState::Ended
    };

    //TODO maybe collect rent on this (need a dummy account for student to create a review later on)
    //TODO cleanup teachers schedule

    //TODO move funds to teacher?

    emit!(LessonEnded {
        teacher_id,
        lesson_id,
        student_id
    });

    Ok(())
}
