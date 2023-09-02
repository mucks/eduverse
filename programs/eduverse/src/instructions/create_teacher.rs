use crate::errors;
use crate::state::{Config, ProfileById, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct TeacherCreated {
    teacher: Pubkey,
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

pub fn handler(
    ctx: Context<CreateTeacher>,
    title: String,
    website: String,
    telegram: String,
    twitter: String,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Store data of teacher
    let teacher = &mut ctx.accounts.teacher_profile;
    teacher.profile_id = config.count_teachers;
    teacher.title = title;
    teacher.availability = 0; //TODO
    teacher.timezone = 0; //TODO
    teacher.website = website;
    teacher.telegram = telegram;
    teacher.twitter = twitter;
    teacher.count_reviews = 0;
    teacher.count_stars = 0;
    teacher.count_lessons = 0;

    // Store teacher profile_id to pubkey lookup
    let teacher_by_id = &mut ctx.accounts.teacher_by_id;
    teacher_by_id.profile_key = teacher.key();

    // Increase number of teacher profiles
    config.count_teachers = config
        .count_teachers
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(TeacherCreated {
        teacher: teacher.key(),
    });

    Ok(())
}
