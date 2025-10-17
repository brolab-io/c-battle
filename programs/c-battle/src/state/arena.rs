use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Arena {
    pub id: u64,
    pub creator: Pubkey,
    pub duration: u32,
    pub max_players: u8,
    pub current_players: u8,
    pub status: u8, // e.g., 0 = waiting, 1 = active, 2 = completed
    pub bump: u8,
}
