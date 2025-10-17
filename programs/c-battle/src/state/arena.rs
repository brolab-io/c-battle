use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Arena {
    pub id: u64,
    pub creator: Pubkey,
    pub duration: u32,
    pub max_players: u8,
    pub bump: u8,
}
