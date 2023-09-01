use anchor_lang::prelude::*;

#[event]
pub struct FundsWithdrawn {
    teacher: u32,
}

#[derive(Accounts)]
#[instruction(
teacher: u32,
amount: u64,
)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Withdraw>, teacher: u32, amount: u64) -> Result<()> {
    //TODO a way for the teacher to withdraw funds from their treasury

    emit!(FundsWithdrawn { teacher });

    Ok(())
}
