use anchor_lang::prelude::*;

declare_id!("HmHG2JRTAVdsBZ6hibDaL9Px1q6afidMhL1E9QfJzUzd");

#[program]
pub mod music3_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
