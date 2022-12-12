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

    pub fn liquidate(ctx: Context<Liquidate>, amount: u64) -> Result<()> {
        let liquidity_acc = &mut ctx.accounts.liquidity_acc;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock_sysvar = &ctx.accounts.clock_sysvar;
        let stake_history_sysvar = &ctx.accounts.stake_history_sysvar;

        invoke_signed(
            &Stake::instruction::withdraw(
                &stake_account.key(),
                &liquidity_acc.key(),
                &liquidity_acc.key(),
                amount,
                None,
            ),
            &[
                stake_account.to_account_info(),
                liquidity_acc.to_account_info(),
                clock_sysvar.to_account_info(),
                stake_history_sysvar.to_account_info(),
            ],
            &[&[b"liquidity-account", &[liquidity_acc.bump]]],
        )?;

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
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"liquidity-account", user.key().as_ref()], bump = liquidity_acc.bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    /// CHECK: validated inside instruction
    pub stake_program: AccountInfo<'info>,
    #[account(mut, seeds = [b"liquidity-account", user.key().as_ref()], bump = liquidity_acc.bump)]
    pub liquidity_acc: Account<'info, LiquidityAccount>,
    /// CHECK: if it was the wrong account tx simply fails
    pub stake_account: AccountInfo<'info>,
    user: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub stake_history_sysvar: Sysvar<'info, StakeHistory>,
}

#[account]
pub struct LiquidityAccount {
    commission: u16, // 2
    bump: u8,        // 1
    balance: u64,    // 8
}
