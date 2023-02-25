use crate::*;

#[derive(Accounts)]
pub struct InitializePocketTradeRegistryContext<'info> {
    // Pocket accounts
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(
        seeds = [PLATFORM_SEED],
        bump = pocket_registry.bump,
    )]
    pub pocket_registry: Account<'info, PocketPlatformRegistry>,

    /// CHECK: skip check
    pub market: AccountInfo<'info>,

    /// CHECK: skip check
    #[account(
        init,
        seeds = [
            market.key().as_ref(),
            pocket.id.as_bytes().as_ref()
        ],
        space = 10240,
        payer = signer,
        bump
    )]
    pub open_orders: AccountInfo<'info>,

    /// CHECK: skip check
    pub dex_program: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializePocketTradeRegistryContext<'info> {
    pub fn execute(&mut self) -> Result<()> {
        let pocket_registry = self.pocket_registry.clone();
        let pocket = self.pocket.clone();
        let signer = self.signer.clone();

        // Only allow operator to perform the swap
        if !pocket_registry.is_operator(signer.key()) {
            return Err(PocketError::OnlyOperator.into());
        }

        // Check whether the pocket is ready to swap
        if !pocket.is_ready_to_swap() {
            return Err(PocketError::NotReadyToSwap.into());
        }

        // Init open orders
        init_open_orders(
            CpiContext::new_with_signer(
                self.dex_program.to_account_info(),
                InitOpenOrders {
                    open_orders: self.open_orders.to_account_info(),
                    authority: self.pocket.to_account_info(),
                    market: self.market.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                &[&[
                    POCKET_SEED,
                    pocket.id.as_bytes().as_ref(),
                    &[pocket.bump],
                ]],
            )
        ).unwrap();

        Ok(())
    }
}