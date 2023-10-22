use crate::errors;
use crate::state::{ProfileById, Review, ReviewById, Student, Teacher};
use anchor_lang::prelude::*;

#[event]
pub struct TeacherReviewCreated {
    teacher_id: u32,
    student_id: u32,
    account_key_review: Pubkey,
    stars: u8,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
student_id: u32,
stars: u8,
text: String,
)]
pub struct CreateReviewTeacher<'info> {
    #[account(mut, constraint = payer.key() == student_profile.authority @errors::ErrorCode::NotAuthorized)]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    seeds = ["student_by_id".as_bytes(), &student_id.to_le_bytes()],
    bump
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = student_by_id.profile_key)]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    init,
    seeds = ["review".as_bytes(), teacher_profile.key().as_ref(), &student_profile.key().as_ref()],
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

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateReviewTeacher>,
    teacher_id: u32,
    student_id: u32,
    stars: u8,
    text: String,
) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;
    let student_profile = &mut ctx.accounts.student_profile;

    // Make sure the student can review this teacher
    if !student_profile.reviewable_teacher_remove(teacher_id) {
        return Err(errors::ErrorCode::LessonNotFunded.into()); //TODO add some error
    } //TODO: currently review PDA would be initializeable only once - figure logic for how many reviews 1 student can give 1 teacher etc

    // Store data for the review
    let review = &mut ctx.accounts.review;
    review.stars = stars;
    review.text = text;

    // Increase number of teachers reviews
    teacher_profile.count_reviews = teacher_profile
        .count_reviews
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Increase total number of stars for teacher
    teacher_profile.count_stars = teacher_profile
        .count_stars
        .checked_add(stars as u32)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(TeacherReviewCreated {
        teacher_id,
        student_id,
        account_key_review: ctx.accounts.review.key(),
        stars
    });

    Ok(())
}
