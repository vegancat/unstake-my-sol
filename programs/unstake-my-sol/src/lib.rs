use anchor_lang::prelude::*;

declare_id!("2s5Phqos3TVKh6CyuFqm8vQukzxueJfMWgtci224h9qc");

#[program]
pub mod unstake_my_sol {
    use super::*;

    pub fn create_liquidity_acc(ctx: Context<CreateLiquidityAcc>) -> Result<()> {
        Ok(())
    }

    pub fn update_commission(ctx: Context<UpdateCommission>, commission: u16) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let user = &mut ctx.accounts.user;

        require_keys_eq!(user.key(), liquidity_acc.owner);
        liquidity_acc.commission = commission;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        Ok(())
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLiquidityAcc<'info> {
    #[account(init, payer = user, space = 8 + 2 + 32)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCommission<'info> {
    #[account(mut)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit {}

#[derive(Accounts)]
pub struct Withdraw {}

#[derive(Accounts)]
pub struct Swap {}

#[derive(Accounts)]
pub struct Liquidate {}

#[account]
pub struct LiquidityAccount {
    commission: u16, // 2
    owner: Pubkey,   // 32
}
