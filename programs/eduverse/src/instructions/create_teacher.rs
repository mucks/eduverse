use crate::errors;
use crate::state::Config;
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
    pub teacher: Box<Account<'info, Teacher>>,

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
    let teacher = &mut ctx.accounts.teacher;
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
