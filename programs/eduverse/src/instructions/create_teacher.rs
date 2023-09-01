use anchor_lang::prelude::*;

use crate::state::teacher::Teacher;

#[event]
pub struct TeacherCreated {
    teacher: Pubkey,
}

#[derive(Accounts)]
#[instruction(
title: String,
website: String,
)]
pub struct CreateTeacher<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = [b"eduverse_teacher", payer.key().as_ref()],
    bump,
    payer = payer,
    space = Teacher::LEN
    )]
    pub teacher: Box<Account<'info, Teacher>>,
    ///TODO setup should depend on how users search for teachers I suppose
    /// okay to have a list all option by using global conf numeration
    /// but need to filter by taught subjects probably?
    ///   could have 3 subjects per teacher
    ///   could use accounts
    ///     by_subject_[subject_id]_[teacher_num]
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTeacher>, title: String, website: String) -> Result<()> {
    // Store data
    let teacher = &mut ctx.accounts.teacher;
    teacher.title = title;
    teacher.website = website;

    emit!(TeacherCreated {
        teacher: teacher.key(),
    });

    Ok(())
}
