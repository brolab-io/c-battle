use anchor_lang::prelude::*;

use crate::{BattleConfig, CONFIG_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + BattleConfig::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub battle_config: Account<'info, BattleConfig>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub power_up_base_cost: u64,
    pub repawn_cost: u64,
    pub max_players_per_arena: u8,
    pub battle_duration_secs: u32,
    pub elimination_reward: u64,
}

pub fn handler(
    ctx: Context<Initialize>,
    power_up_base_cost: u64,
    repawn_cost: u64,
    max_players_per_arena: u8,
    battle_duration_secs: u32,
    elimination_reward: u64,
) -> Result<()> {
    ctx.accounts.battle_config.set_inner(BattleConfig {
        authority: ctx.accounts.signer.key(),
        power_up_base_cost,
        repawn_cost,
        max_players_per_arena,
        battle_duration_secs,
        elimination_reward,
        bump: ctx.bumps.battle_config,
    });

    emit!(ConfigInitialized {
        authority: ctx.accounts.signer.key(),
        power_up_base_cost,
        repawn_cost,
        max_players_per_arena,
        battle_duration_secs,
        elimination_reward,
    });
    Ok(())
}
