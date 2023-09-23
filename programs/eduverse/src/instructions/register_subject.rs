use anchor_lang::prelude::*;
use crate::errors;
use crate::state::{Teacher, SubjectTeacher, SubjectConfig, TeacherSubject};

#[event]
pub struct SubjectRegistered {
    teacher_profile: u32,
    subject_id: u32,
}

#[derive(Accounts)]
#[instruction(
subject_id: u32,
)]
pub struct RegisterSubject<'info> {
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

    #[account(
    init,
    seeds = ["teacher_subject".as_bytes(), &teacher_profile.key().as_ref(), &teacher_profile.count_subjects.to_le_bytes()],
    bump,
    payer = payer,
    space = TeacherSubject::LEN
    )]
    pub teacher_subject: Box<Account<'info, TeacherSubject>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterSubject>, subject_id: u32) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;

    // Store the teachers profile_id in the subject to teacher lookup account
    let subject_teacher = &mut ctx.accounts.subject_teacher;
    subject_teacher.teacher_profile_id = teacher_profile.profile_id;

    // Increment number of teachers for the subject
    let subject_config = &mut ctx.accounts.subject_config;
    subject_config.count_teachers = subject_config
        .count_teachers
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store the subject in the teacher to subject lookup account
    let teacher_subject = &mut ctx.accounts.teacher_subject;
    teacher_subject.subject_id = subject_id;

    // Increase the number of subjects taught by this teacher
    teacher_profile.count_subjects = teacher_profile
        .count_subjects
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // TODO there is no safeguard against teachers registering the same subject many times (except them wasting money on rent)
    // another account? teacher_profile.key() : subject_id
    // frontend could do a soft check, can also add fees :) might be good for a marketing budget

    emit!(SubjectRegistered {
        teacher_profile: teacher_profile.profile_id,
        subject_id
    });

    Ok(())
}
