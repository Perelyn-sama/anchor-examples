use anchor_lang::prelude::*;

declare_id!("C4hkbSVcQHo7bLiZe1jpi1s5dpefSRPHtC4EYitYDcs");

#[program]
pub mod clock {
    use super::*;

    pub fn get_clock(ctx: Context<GetClock>) -> Result<()> {
        let clock_state = &mut ctx.accounts.clock_state;
        let clock = Clock::get()?;

        clock_state.slot = clock.slot;
        clock_state.epoch_start_timestamp = clock.epoch_start_timestamp;
        clock_state.epoch = clock.epoch;
        clock_state.leader_schedule_epoch = clock.leader_schedule_epoch;
        clock_state.leader_schedule_epoch = clock.leader_schedule_epoch;
        clock_state.unix_timestamp = clock.unix_timestamp as u64;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetClock<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + std::mem::size_of::<ClockState>())]
    pub clock_state: Account<'info, ClockState>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClockState {
    pub slot: u64,
    pub epoch_start_timestamp: i64,
    pub epoch: u64,
    pub leader_schedule_epoch: u64,
    pub unix_timestamp: u64,
}

// https://docs.rs/solana-program/1.17.14/solana_program/clock/index.html
// https://github.com/solana-labs/solana-program-library/tree/master/examples/rust/sysvar
