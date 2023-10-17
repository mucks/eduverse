use crate::errors;
use crate::state::{Lesson, ProfileById, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;

#[event]
pub struct LessonApproved {
    teacher_id: u32,
    lesson_id: u32,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
lesson_id: u32,
)]
pub struct LessonApprove<'info> {
    #[account(address = teacher_profile.authority)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    mut,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &lesson_id.to_le_bytes()],
    bump,
    constraint = lesson.status_teacher == LessonState::Pending @errors::ErrorCode::LessonStateNotPending
    )]
    pub lesson: Box<Account<'info, Lesson>>,
}

/// Approve the lesson. Only for teachers. Students should deposit the funds for the lesson in order to "approve" it
pub fn handler(ctx: Context<LessonApprove>, teacher_id: u32, lesson_id: u32) -> Result<()> {
    let lesson = &mut ctx.accounts.lesson;

    // Update the teachers approval state for this lesson
    lesson.status_teacher = LessonState::Approved;

    emit!(LessonApproved {
        teacher_id,
        lesson_id
    });

    Ok(())
}
