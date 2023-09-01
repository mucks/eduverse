use crate::errors;
use crate::state::Config;
use anchor_lang::prelude::*;

use crate::state::student::Student;

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
    pub student: Box<Account<'info, Student>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateStudent>, title: String, contact_info: String) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Store data of student
    let student = &mut ctx.accounts.student;
    student.profile_id = config.count_students;
    student.title = title;
    student.contact_info = contact_info;

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
