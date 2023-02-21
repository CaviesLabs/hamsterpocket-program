use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::dex::{new_order_v3, settle_funds, SettleFunds, serum_dex::{instruction::SelfTradeBehavior, matching::{OrderType, Side}, }, NewOrderV3 };

pub mod action;
pub mod error;
pub mod event;
pub mod state;
pub mod constants;
pub mod macros;

pub use action::*;
pub use constants::*;
pub use error::*;
pub use state::*;
pub use event::*;
pub use macros::*;

declare_id!("DL1BQwRmN4Ye4fsmDnPipNrm1XU24x8yDU7c5Ltsqvvc");

#[program]
pub mod pocket {
    use super::*;

    // Initialize contract once
    pub fn initialize(
        ctx: Context<InitializePocketPlatformContext>,
        params: InitializePocketPlatformParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(
            params,
            *ctx.bumps.get("pocket_registry").unwrap(),
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn update_pocket_registry(
        ctx: Context<UpdatePocketRegistryContext>,
        params: UpdatePocketRegistryParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(params).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn create_token_vault(
        ctx: Context<CreateTokenVaultContext>,
    ) -> Result<()> {
        // process
        ctx.accounts.execute().unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn create_pocket(
        ctx: Context<CreatePocketContext>,
        params: CreatePocketParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(
            params,
            *ctx.bumps.get("pocket").unwrap(),
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn update_pocket(
        ctx: Context<UpdatePocketContext>,
        params: UpdatePocketParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(params).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn deposit(
        ctx: Context<DepositContext>,
        params: DepositParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(
            params
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn withdraw(
        ctx: Context<WithdrawContext>,
        params: WithdrawParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(
            params
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

}