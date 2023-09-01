use crate::errors;
use anchor_lang::prelude::*;

use crate::state::lesson::Lesson;
use crate::state::{Review, Teacher};

#[event]
pub struct ReviewCreated {
    account: Pubkey,
    teacher: Pubkey,
    stars: u8,
}

#[derive(Accounts)]
#[instruction(
stars: u8,
text: String,
)]
pub struct CreateLesson<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub lesson: Account<'info, Lesson>,

    #[account(mut)]
    pub teacher: Account<'info, Teacher>,

    //TODO need teacher & user reviews
    #[account(
    init,
    seeds = ["review".as_bytes(), teacher.key().as_ref(), &teacher.count_reviews.to_le_bytes()],
    bump,
    payer = payer,
    space = Review::LEN
    )]
    pub review: Box<Account<'info, Review>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateLesson>, stars: u8, text: String) -> Result<()> {
    //TODO make sure the lesson took place etc.

    //Todo figure out how many lessons reviewer had?
    //  probably in front-end
    //   teacher _ lessun_num _ iterate, check student
    //   or statistics accounts teacher : student -> stats (total time learned, number of lessons, ...?)

    // Store data for the review
    let review = &mut ctx.accounts.review;
    review.stars = stars;
    review.text = text;

    // Increase number of teachers reviews
    let teacher = &mut ctx.accounts.teacher;
    teacher.count_reviews = teacher
        .count_reviews
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Increase total number of stars for teacher
    let teacher = &mut ctx.accounts.teacher;
    teacher.count_stars = teacher
        .count_stars
        .checked_add(stars as u32)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(ReviewCreated {
        account: ctx.accounts.review.key(),
        teacher: ctx.accounts.teacher.key(),
        stars
    });

    Ok(())
}
