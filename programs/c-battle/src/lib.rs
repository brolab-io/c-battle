pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FXBqApebbiqi4G3kGobVSfV3uNYY8Cwe26oocZX4CGgC");

#[program]
pub mod c_battle {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        power_up_base_cost: u64,
        repawn_cost: u64,
        max_players_per_arena: u8,
        battle_duration_secs: u32,
        elimination_reward: u64,
    ) -> Result<()> {
        initialize::handler(
            ctx,
            power_up_base_cost,
            repawn_cost,
            max_players_per_arena,
            battle_duration_secs,
            elimination_reward,
        )
    }

    pub fn make_arena(
        ctx: Context<MakeArena>,
        id: u64,
        max_players: u8,
        duration: u32,
    ) -> Result<()> {
        make_area::handler(ctx, id, max_players, duration)
    }
}
