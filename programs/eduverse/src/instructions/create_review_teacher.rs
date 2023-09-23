use crate::errors;
use anchor_lang::prelude::*;

use crate::state::lesson::Lesson;
use crate::state::{Review, ReviewById, Student, Teacher};

#[event]
pub struct TeacherReviewCreated {
    account: Pubkey,
    teacher: Pubkey,
    stars: u8,
}

#[derive(Accounts)]
#[instruction(
lesson_num: u32,
text: String,
)]
pub struct CreateReviewTeacher<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["student".as_bytes(), payer.key().as_ref()],
    bump
    )]
    pub student_profile: Box<Account<'info, Student>>,

    pub teacher_profile: Account<'info, Teacher>,

    #[account(
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &lesson_num.to_le_bytes()],
    bump
    )]
    pub lesson: Account<'info, Lesson>,

    #[account(
    init,
    seeds = ["review".as_bytes(), teacher_profile.key().as_ref(), &student_profile.profile_id.to_le_bytes()],
    bump,
    payer = payer,
    space = Review::LEN
    )]
    pub review: Box<Account<'info, Review>>,

    #[account(
    init,
    seeds = ["review_lookup".as_bytes(), teacher_profile.key().as_ref(), &teacher_profile.count_reviews.to_le_bytes()],
    bump,
    payer = payer,
    space = ReviewById::LEN
    )]
    pub review_lookup: Box<Account<'info, ReviewById>>,

    //TODO need a blocker account for teacher_profile.key + student_profile.key so as to only have one review per user
    // or do lookup accounts teacher_profile.key : num -> review_acc.key or student_profile_id ("review" _ teacher.key _ student.key)

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateReviewTeacher>, stars: u8, text: String) -> Result<()> {
    //TODO make sure the lesson took place etc.

    // Store data for the review
    let review = &mut ctx.accounts.review;
    review.stars = stars;
    review.text = text;

    // Increase number of teachers reviews
    let teacher = &mut ctx.accounts.teacher_profile;
    teacher.count_reviews = teacher
        .count_reviews
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Increase total number of stars for teacher
    teacher.count_stars = teacher
        .count_stars
        .checked_add(stars as u32)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(TeacherReviewCreated {
        account: ctx.accounts.review.key(),
        teacher: ctx.accounts.teacher_profile.key(),
        stars
    });

    Ok(())
}
