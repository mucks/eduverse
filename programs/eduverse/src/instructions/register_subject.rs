use anchor_lang::prelude::*;

use crate::state::subject_teacher::SubjectTeacher;

#[event]
pub struct SubjectRegistered {
    teacher: u32,
    subject: u32,
}

#[derive(Accounts)]
#[instruction(
subject: u32,
)]
pub struct RegisterSubject<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = [b"filter_subject", payer.key().as_ref()],
    bump,
    payer = payer,
    space = SubjectTeacher::LEN
    )]
    pub subject_teacher: Box<Account<'info, SubjectTeacher>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterSubject>, subject: u32) -> Result<()> {
    // Store data
    let subject_teacher = &mut ctx.accounts.subject_teacher;
    subject_teacher.teacher_profile = 0;

    //TODO init_if_needed on subject account

    emit!(SubjectRegistered {
        teacher: 0,
        subject
    });

    Ok(())
}
