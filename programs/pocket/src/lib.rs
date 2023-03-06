use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};

use anchor_spl::{
    token::{self, Token, TokenAccount, Transfer, Mint},
};

pub mod action;
pub mod error;
pub mod event;
pub mod state;
pub mod constants;
pub mod macros;
pub mod external;

pub use action::*;
pub use constants::*;
pub use error::*;
pub use state::*;
pub use event::*;
pub use macros::*;
pub use external::*;


declare_id!("BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV");

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
    ) -> Result<()> {
        // process
        ctx.accounts.execute().unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn close_pocket_accounts(
        ctx: Context<ClosePocketAccountsContext>,
    ) -> Result<()> {
        // process
        ctx.accounts.execute().unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn init_swap_registry<'info>(
        ctx: Context<'_, '_, '_, 'info, InitAccount<'info>>
    ) -> Result<()> {

        // Init open orders account
        init_account(
            &InitAccount {
                open_orders: ctx.accounts.open_orders.to_account_info(),
                dex_program: ctx.accounts.dex_program.to_account_info(),
                authority: ctx.accounts.pocket.to_account_info(),
                market_key: ctx.accounts.market_key.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                pocket: ctx.accounts.pocket.clone(),
            }
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn close_swap_registry<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseAccount<'info>>
    ) -> Result<()> {

        // Close open orders account
        close_account(
            &CloseAccount {
                open_orders: ctx.accounts.open_orders.to_account_info(),
                dex_program: ctx.accounts.dex_program.to_account_info(),
                authority: ctx.accounts.pocket.to_account_info(),
                destination: ctx.accounts.destination.to_account_info(),
                market_key: ctx.accounts.market_key.to_account_info(),
                pocket: ctx.accounts.pocket.clone(),
            }
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }

    pub fn execute_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>
    ) -> Result<()> {
        // process
        handle_execute_swap(ctx).unwrap();

        // Program result should be ok.
        Ok(())
    }

}