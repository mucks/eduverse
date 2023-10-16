#[cfg(feature = "anchor_contract")]
use anchor_lang::declare_id;

pub mod enums;
pub mod state;
pub mod structs;

#[cfg(feature = "anchor_contract")]
declare_id!("AihTskBQM3txbtFMx4awbZrMLsyiVE17LvBs7hskq1W");
