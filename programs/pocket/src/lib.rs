use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::token::{Mint, Token, TokenAccount};

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

declare_id!("k4odSWqFPwacCdZVUwngYiWMDugEawGGvZCLfcRrsBf");

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


    // Initialize contract once
    pub fn create_pocket(
        ctx: Context<CreatePocketContext>,
        params: CreatePocketParams
    ) -> Result<()> {
        // process
        ctx.accounts.execute(
            params,
            *ctx.bumps.get("pocket").unwrap(),
            *ctx.bumps.get("pocket_token_vault").unwrap(),
        ).unwrap();

        // Program result should be ok.
        Ok(())
    }
}