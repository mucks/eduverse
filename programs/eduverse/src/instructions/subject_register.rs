use crate::errors;
use crate::state::{SubjectConfig, SubjectTeacher, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct SubjectRegistered {
    teacher_profile: u32,
    subject_id: u32,
}

#[derive(Accounts)]
#[instruction(
subject_id: u32,
)]
pub struct SubjectRegister<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = ["teacher".as_bytes(), payer.key().as_ref()],
    bump
    )]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    init_if_needed,
    seeds = ["subject_config".as_bytes(), &subject_id.to_le_bytes()],
    bump,
    payer = payer,
    space = SubjectConfig::LEN,
    )]
    pub subject_config: Box<Account<'info, SubjectConfig>>,

    #[account(
    init,
    seeds = ["subject_teacher".as_bytes(), &subject_id.to_le_bytes(), &subject_config.count_teachers.to_le_bytes()],
    bump,
    payer = payer,
    space = SubjectTeacher::LEN
    )]
    pub subject_teacher: Box<Account<'info, SubjectTeacher>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

/// A teacher can register a certain subject that they want to teach. If necessary the subject config account will also be created.
pub fn handler(ctx: Context<SubjectRegister>, subject_id: u32) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;

    // Add this subject to subjects taught by this teacher
    if !teacher_profile.add_subject(subject_id) {
        return Err(errors::ErrorCode::SubjectLimitReached.into());
    }

    // Store the teachers profile_id in the subject to teacher lookup account
    let subject_teacher = &mut ctx.accounts.subject_teacher;
    subject_teacher.teacher_profile_id = teacher_profile.profile_id;

    // Increment number of teachers for the subject
    let subject_config = &mut ctx.accounts.subject_config;
    subject_config.count_teachers = subject_config
        .count_teachers
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(SubjectRegistered {
        teacher_profile: teacher_profile.profile_id,
        subject_id
    });

    Ok(())
}
