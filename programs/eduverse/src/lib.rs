use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
mod utils;

use crate::instructions::*;

declare_id!("AihTskBQM3txbtFMx4awbZrMLsyiVE17LvBs7hskq1W");

#[program]
pub mod eduverse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    /// Create a new teacher profile
    pub fn create_teacher(
        ctx: Context<CreateTeacher>,
        title: String,
        website: String,
        telegram: String,
        twitter: String,
    ) -> Result<()> {
        create_teacher::handler(ctx, title, website, telegram, twitter)
    }
}
