use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BattleConfig {
    pub authority: Pubkey,
    pub power_up_base_cost: u64,
    pub repawn_cost: u64,
    pub max_players_per_arena: u8,
    pub battle_duration_secs: u32,
    pub elimination_reward: u64,
    pub bump: u8,
}
