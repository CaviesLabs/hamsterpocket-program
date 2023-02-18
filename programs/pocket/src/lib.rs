use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

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

declare_id!("EdeRcNsVGU1s1NXZZo8FhLD8iePxvoUCdbvwVGnj778f");

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
}