use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod unstake_my_sol {
    use super::*;

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
pub struct Deposit {}

#[derive(Accounts)]
pub struct Withdraw {}

#[derive(Accounts)]
pub struct Swap {}

#[derive(Accounts)]
pub struct Liquidate {}
