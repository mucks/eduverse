use anchor_lang::prelude::*;

#[event]
pub struct LessonStarted {
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
    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<LessonBegin>,
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
) -> Result<()> {
    //TODO must have been approved, must have been funded, correct time, ...
    emit!(LessonStarted {
        teacher_id,
        lesson_id,
        student_id
    });

    Ok(())
}
