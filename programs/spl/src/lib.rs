use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};


declare_id!("2Bh3qNT7Nqfk6LHV3mAj7J6aEZPb8JdACqtadSu1NEJU");

#[program]
pub mod spl {
    use super::*;

    pub fn spl_methods(ctx: Context<Spl>) -> Result<()> {

        let placeholder_amount = 0;

        // spl transfer
        token::transfer(ctx.accounts.transfer_context(), placeholder_amount)?;

        // spl burn
        token::burn(
            ctx.accounts.burn_context(),
            placeholder_amount,
        )?;


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Spl<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
}

impl<'info> Spl<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.sender_token_account.to_account_info(),
                to: self.receiver_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.mint.to_account_info(),
                from: self.sender_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}
