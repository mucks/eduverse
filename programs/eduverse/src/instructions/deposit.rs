use crate::state::{Lesson, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct FundsDeposited {
    teacher: u32,
    lesson: u32,
}

#[derive(Accounts)]
#[instruction(
teacher: u32,
lesson: u32,
)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub lesson: Account<'info, Lesson>,

    pub teacher: Account<'info, Teacher>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, teacher: u32, lesson: u32) -> Result<()> {
    //TODO transfer SOL to a treasury account for the teacher

    emit!(FundsDeposited { teacher, lesson });

    Ok(())
}
