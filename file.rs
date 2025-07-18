use anchor_lang::prelude::*;

#[account] 
#[derive(Accounts)]
pub struct Lending {
  pub lender: Pubkey,
  pub borrower: Pubkey,
  pub amount: u64, //@note :: dummy flaw that will be fixed by deeydos via PR to u64
  pub interest_bps: u16,
  pub borrowed_date: i64,
  pub grace_period: i64,
}
