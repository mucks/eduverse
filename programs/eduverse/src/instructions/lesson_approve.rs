use anchor_lang::prelude::*;

#[event]
pub struct LessonApproved {
    teacher_id: u32,
    student_id: u32,
    lesson_id: u32,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
student_id: u32,
lesson_id: u32,
)]
pub struct LessonApprove<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,

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

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<LessonApprove>, teacher_id: u32, student_id: u32, lesson_id: u32) -> Result<()> {
    emit!(LessonApproved {
        teacher_id,
        lesson_id,
        student_id
    });

    Ok(())
}
