use anchor_lang::prelude::*;

use crate::state::lesson::Lesson;

#[event]
pub struct LessonCreated {
    teacher: Pubkey,
    student: u32,
}

#[derive(Accounts)]
#[instruction(
student: u32,
)]
pub struct CreateLesson<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = [b"eduverse_lesson", payer.key().as_ref()],
    bump,
    payer = payer,
    space = Lesson::LEN
    )]
    pub lesson: Box<Account<'info, Lesson>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateLesson>, student: u32) -> Result<()> {
    // Store data
    let lesson = &mut ctx.accounts.lesson;
    lesson.student = student;

    emit!(LessonCreated {
        teacher: ctx.accounts.payer.key(),
        student
    });

    Ok(())
}
