use crate::state::Lesson;
use anchor_lang::prelude::*;

use crate::state::review::Review;

#[event]
pub struct LessonCreated {
    teacher: Pubkey,
}

#[derive(Accounts)]
#[instruction(
student: u8,
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

pub fn handler(ctx: Context<CreateLesson>, rating: u8) -> Result<()> {
    // Store data
    let lesson = &mut ctx.accounts.lesson;
    lesson.timestamp = 0;
    lesson.duration = 0;
    lesson.fee_total = 0;
    lesson.fee_deposited = 0;
    lesson.repeat = 0;
    lesson.cancel = 0;
    lesson.student = 0;
    lesson.subject = 0;

    emit!(LessonCreated {
        teacher: ctx.accounts.payer.key(),
    });

    Ok(())
}
