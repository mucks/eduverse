use anchor_lang::prelude::*;

use crate::state::Config;

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = [b"config"],
    bump,
    payer = payer,
    space = Config::LEN
    )]
    pub config: Account<'info, Config>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

/// Initialize the config for the app. This does not currently require any permissions, as there is no admin.
pub fn handler(_ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
