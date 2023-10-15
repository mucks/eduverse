use crate::errors;
use crate::state::{Lesson, ProfileById, Student, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;

#[event]
pub struct LessonClosed {
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
pub struct LessonClose<'info> {
    #[account(mut, constraint = payer.key() == teacher_profile.authority @errors::ErrorCode::NotAuthorized)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    seeds = ["student_by_id".as_bytes(), &student_id.to_le_bytes()],
    bump
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = student_by_id.profile_key)]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    mut,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &lesson_id.to_le_bytes()],
    bump,
    constraint = lesson.student_id == student_id,
    constraint = lesson.status_teacher == LessonState::Started @errors::ErrorCode::LessonStateNotStarted,
    close = payer
    )]
    pub lesson: Box<Account<'info, Lesson>>,
}

pub fn handler(
    ctx: Context<LessonClose>,
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;
    let student_profile = &mut ctx.accounts.student_profile;

    //TODO check time is after conclusion? Does it matter?

    // Cleanup the teachers schedule
    teacher_profile.schedule_remove(lesson_id);

    // Cleanup the students schedule
    student_profile.schedule_remove(teacher_id, lesson_id);

    // Mark this teacher as reviewable by this student
    student_profile.reviewable_teacher_add(teacher_id);

    //TODO Stats that may need updating?

    emit!(LessonClosed {
        teacher_id,
        lesson_id,
        student_id
    });

    Ok(())
}
