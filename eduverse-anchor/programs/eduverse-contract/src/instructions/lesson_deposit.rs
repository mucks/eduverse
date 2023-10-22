use crate::errors;
use crate::state::{Lesson, ProfileById, Student, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

#[event]
pub struct LessonFundsDeposited {
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
    total_fee: u64,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
lesson_id: u32,
student_id: u32,
)]
pub struct LessonDeposit<'info> {
    #[account(mut, constraint = payer.key() == student_profile.authority @errors::ErrorCode::NotAuthorized)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    seeds = ["student_by_id".as_bytes(), &student_id.to_le_bytes()],
    bump
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    #[account(address = student_by_id.profile_key)]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    mut,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &lesson_id.to_le_bytes()],
    bump,
    constraint = lesson.student_id == student_id,
    constraint = lesson.status_teacher == LessonState::Approved @errors::ErrorCode::LessonStateNotApproved
    )]
    pub lesson: Box<Account<'info, Lesson>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<LessonDeposit>,
    teacher_id: u32,
    lesson_id: u32,
    student_id: u32,
) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let lesson_key = ctx.accounts.lesson.key();
    let lesson = &mut ctx.accounts.lesson;
    let total_fee = lesson.fee_total;

    //TODO should anyone other than the student be able to fund their lesson?

    // Check for token transfers
    if lesson.token_address != None {
        //TODO handle SPL tokens such as BONK & USDC & EURe
    }

    // Transfer the funds if necessary
    if total_fee > 0 {
        // Setup the fund transferring instruction
        let transfer = system_instruction::transfer(payer.key, &lesson_key, total_fee);

        // Invoke the actual transfer
        anchor_lang::solana_program::program::invoke_signed(
            &transfer,
            &[
                payer.to_account_info(),
                lesson.clone().to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
    }

    // Mark funds as deposited and lesson as approved by the student
    lesson.fee_deposited = total_fee; //TODO make sure funds were transferred etc.
    lesson.status_student = LessonState::Approved;

    emit!(LessonFundsDeposited {
        teacher_id,
        lesson_id,
        student_id,
        total_fee
    });

    Ok(())
}
