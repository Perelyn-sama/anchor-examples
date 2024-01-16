use anchor_lang::prelude::*;

declare_id!("jdKokT2sUSDzQF2AFZ2ZzLbemj89ZDdFh2abR7xzjc2");

#[program]
pub mod anchor_examples {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
