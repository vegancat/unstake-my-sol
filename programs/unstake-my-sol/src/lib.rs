use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    stake as Stake, system_instruction, vote as Vote,
};
use anchor_lang::system_program;

declare_id!("2s5Phqos3TVKh6CyuFqm8vQukzxueJfMWgtci224h9qc");

#[program]
pub mod unstake_my_sol {
    use super::*;

    pub fn create_liquidity_acc(ctx: Context<CreateLiquidityAcc>, commission: u16) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let bump = *ctx.bumps.get("liquidity-account").unwrap();

        liquidity_acc.commission = commission;
        liquidity_acc.balance = 0;
        liquidity_acc.bump = bump;
        Ok(())
    }

    pub fn update_commission(ctx: Context<UpdateCommission>, commission: u16) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;

        liquidity_acc.commission = commission;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let system_program_account_info = &ctx.accounts.system_program.to_account_info();

        liquidity_acc.balance += amount;

        invoke(
            &system_instruction::transfer(&user.key(), &liquidity_acc.key(), amount),
            &[
                user.to_account_info(),
                liquidity_acc.to_account_info(),
                system_program_account_info.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let system_program_account_info = &ctx.accounts.system_program.to_account_info();

        invoke_signed(
            &system_instruction::transfer(&liquidity_acc.key(), &user.key(), amount),
            &[
                liquidity_acc.to_account_info(),
                user.to_account_info(),
                system_program_account_info.clone(),
            ],
            &[&[b"liquidity-account", &[liquidity_acc.bump]]],
        )?;

        liquidity_acc.balance -= amount;
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
    #[account(init, payer = user, space = 8 + 2 + 1 + 8, seeds = [b"liquidity-account", user.key().as_ref()], bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCommission<'info> {
    #[account(mut, seeds = [b"liquidity-account", user.key().as_ref()], bump = liquidity_acc.bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"liquidity-account", user.key().as_ref()], bump = liquidity_acc.bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    pub user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"liquidity-account", user.key().as_ref()], bump = liquidity_acc.bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    pub user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Liquidate {}

#[account]
pub struct LiquidityAccount {
    commission: u16, // 2
    bump: u8,        // 1
    balance: u64,    // 8
}
