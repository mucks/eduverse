use crate::errors;
use crate::state::{Config, ProfileById, Student};
use anchor_lang::prelude::*;

#[event]
pub struct StudentCreated {
    student: Pubkey,
}

#[derive(Accounts)]
#[instruction(
title: String,
contact_info: String,
)]
pub struct CreateStudent<'info> {
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
    seeds = ["student".as_bytes(), payer.key().as_ref()],
    bump,
    payer = payer,
    space = Student::LEN
    )]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    init,
    seeds = ["student_by_id".as_bytes(), &config.count_teachers.to_le_bytes()],
    bump,
    payer = payer,
    space = ProfileById::LEN
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateStudent>, title: String, contact_info: String) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Store data of student
    let student = &mut ctx.accounts.student_profile;
    student.profile_id = config.count_students;
    student.title = title;
    student.contact_info = contact_info;

    // Store student profile_id to pubkey lookup
    let student_by_id = &mut ctx.accounts.student_by_id;
    student_by_id.profile_key = student.key();

    // Increase number of student profiles
    config.count_students = config
        .count_students
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(StudentCreated {
        student: student.key(),
    });

    Ok(())
}
