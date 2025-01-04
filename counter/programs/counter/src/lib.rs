use anchor_lang::prelude::*;

declare_id!("GcGndELtKJsv9WdvCFjm7J2QAPVxLzgSpbgNwdd9K3jQ");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
