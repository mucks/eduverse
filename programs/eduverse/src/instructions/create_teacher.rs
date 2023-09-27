use crate::errors;
use crate::state::{Config, ProfileById, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct TeacherCreated {
    account_key_teacher: Pubkey,
}

#[derive(Accounts)]
#[instruction(
title: String,
website: String,
telegram: String,
twitter: String,
)]
pub struct CreateTeacher<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = [b"config"],
    bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
    init,
    seeds = ["teacher".as_bytes(), payer.key().as_ref()],
    bump,
    payer = payer,
    space = Teacher::LEN
    )]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    init,
    seeds = ["teacher_by_id".as_bytes(), &config.count_teachers.to_le_bytes()],
    bump,
    payer = payer,
    space = ProfileById::LEN
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

/// Create a new teacher profile. The profile is created for the signer of the call. A lookup account is also created.
pub fn handler(
    ctx: Context<CreateTeacher>,
    title: String,
    website: String,
    telegram: String,
    twitter: String,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Store data of teacher
    let teacher_profile = &mut ctx.accounts.teacher_profile;
    teacher_profile.profile_id = config.count_teachers;
    teacher_profile.authority = ctx.accounts.payer.key();
    teacher_profile.registered_at = Clock::get().unwrap().unix_timestamp;
    teacher_profile.title = title;
    teacher_profile.availability = 0; //TODO
    teacher_profile.timezone = 0; //TODO
    teacher_profile.website = website;
    teacher_profile.telegram = telegram;
    teacher_profile.twitter = twitter;
    // Lesson 0 won't be a valid id, since lesson references are stored in an array where 0 means empty
    teacher_profile.count_lessons = 1;

    // Store teacher profile_id to pubkey lookup
    let teacher_by_id = &mut ctx.accounts.teacher_by_id;
    teacher_by_id.profile_key = teacher_profile.key();

    // Increase number of teacher profiles
    config.count_teachers = config
        .count_teachers
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(TeacherCreated {
        account_key_teacher: teacher_profile.key(),
    });

    Ok(())
}
