use anchor_lang::prelude::*;

declare_id!("AihTskBQM3txbtFMx4awbZrMLsyiVE17LvBs7hskq1W");

#[program]
pub mod eduverse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
