use anchor_lang::prelude::*;

declare_id!("GcGndELtKJsv9WdvCFjm7J2QAPVxLzgSpbgNwdd9K3jQ");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);

        let counter = &ctx.accounts.counter;
        msg!("Counter account created! current count : {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // now we're incrementing the counter.count value by 1
        // we're doing from checked_add instead of adding +, becasue to prevent from overflow issues
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented: {}", counter.count);

        Ok(())
    }
}
//type declaration
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    // so in here, we are creating an account and storing in a variable named 'counter'. weher 'info keyword represents the lifetiime
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer= user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    // this is the account which will help us creating an account
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
// pub struct Initialize {}
