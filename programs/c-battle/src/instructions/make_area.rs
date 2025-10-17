use anchor_lang::prelude::*;

use crate::{Arena, ARENA_SEED};


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeArena<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        payer = signer, 
        space = 8 + Arena::INIT_SPACE,
        seeds = [ARENA_SEED, &id.to_le_bytes()],
        bump,
    )]
    pub arena: Account<'info, Arena>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct ArenaCreated {
    pub id: u64,
    pub creator: Pubkey,
    pub duration: u32,
    pub max_players: u8,
    pub status: u8,
}

pub fn handler(ctx: Context<MakeArena>, id: u64, max_players:u8, duration: u32) -> Result<()> {
    ctx.accounts.arena.set_inner(Arena { 
            id,
            creator: ctx.accounts.signer.key(), 
            duration, 
            max_players, 
            bump: ctx.bumps.arena ,
            current_players: 0,
            status: 0,
        }
    );
    emit!(ArenaCreated {
        id,
        creator: ctx.accounts.signer.key(),
        duration,
        max_players,
        status: 0,
    });
    Ok(())
}
