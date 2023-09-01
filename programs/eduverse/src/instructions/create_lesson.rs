use anchor_lang::prelude::*;

use crate::state::review::Review;

#[event]
pub struct ReviewCreated {
    teacher: Pubkey,
    rating: u8,
}

#[derive(Accounts)]
#[instruction(
rating: u8,
)]
pub struct CreateReview<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = [b"eduverse_lesson", payer.key().as_ref()],
    bump,
    payer = payer,
    space = Review::LEN
    )]
    pub review: Box<Account<'info, Review>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateReview>, rating: u8) -> Result<()> {
    // Store data
    let review = &mut ctx.accounts.review;
    review.rating = rating;

    //Todo figure out how many lessons reviewer had?
    //  probably in front-end
    //   teacher _ lessun_num _ iterate, check student
    //   or statistics accounts teacher : student -> stats (total time learned, number of lessons, ...?)

    emit!(ReviewCreated {
        teacher: ctx.accounts.payer.key(),
        rating
    });

    Ok(())
}
