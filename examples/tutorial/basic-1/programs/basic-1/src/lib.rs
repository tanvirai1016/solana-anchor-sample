use anchor_lang::prelude::*;

declare_id!("F41aWthQJGRVGiCQSHLUnREGd9YGJAFmgiBAicohnbjP");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        Ok(())
    }
  
    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn increment(ctx: Context<increment>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data += 1;
        Ok(())
    }
    pub fn decrement(ctx: Context<decrement>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data -= 1;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct increment<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}
#[derive(Accounts)]
pub struct decrement<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
