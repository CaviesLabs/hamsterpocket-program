use crate::*;

#[derive(Accounts)]
pub struct ExecuteSwapContext<'info> {
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

    #[account(mut)]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pocket_target_token_vault: Account<'info, TokenAccount>,

    // Serum Dex Accounts
    /// CHECK: skip check
    pub market: AccountInfo<'info>,
    /// CHECK: skip check
    pub coin_vault: AccountInfo<'info>,
    /// CHECK: skip check
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: skip check
    pub request_queue: AccountInfo<'info>,
    /// CHECK: skip check
    pub event_queue: AccountInfo<'info>,
    /// CHECK: skip check
    pub market_bids: AccountInfo<'info>,
    /// CHECK: skip check
    pub market_asks: AccountInfo<'info>,
    /// CHECK: skip check
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

impl<'info> ExecuteSwapContext<'info> {
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

        // TODO: check for buy condition

        // Make Swap
        self.make_swap().unwrap();

        Ok(())
    }

    fn calculate_quote(&mut self){

    }

    fn make_swap(&mut self) -> Result<()> {
        let pocket = self.pocket.clone();
        let dex_program = self.dex_program.clone();

        let mut side = Side::Bid;

        if pocket.side == TradeSide::Buy {
            side = Side::Ask;
        }

        new_order_v3(
            CpiContext::new_with_signer(
                dex_program.to_account_info(),
                NewOrderV3 {
                    market: self.market.to_account_info(),
                    coin_vault: self.coin_vault.to_account_info(),
                    pc_vault: self.pc_vault.to_account_info(),
                    request_queue: self.request_queue.to_account_info(),
                    event_queue: self.event_queue.to_account_info(),
                    market_bids: self.market_bids.to_account_info(),
                    market_asks: self.market_asks.to_account_info(),
                    open_orders: self.open_orders.to_account_info(),
                    order_payer_token_account: self.pocket_base_token_vault.to_account_info(),
                    open_orders_authority: self.pocket.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                &[&[
                    POCKET_SEED,
                    pocket.id.as_bytes().as_ref(),
                    &[pocket.bump],
                ]],
            ),
            side,
            NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
            NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
            NonZeroU64::new(pocket.batch_volume).unwrap(),
            SelfTradeBehavior::DecrementTake,
            OrderType::ImmediateOrCancel,
            u64::try_from_slice(&pocket.key().to_bytes()[0..8]).unwrap(),
            std::u16::MAX,
        ).unwrap();

        settle_funds(
            CpiContext::new_with_signer(
                dex_program.to_account_info(),
                SettleFunds {
                    market: self.market.to_account_info(),
                    open_orders: self.open_orders.to_account_info(),
                    open_orders_authority: self.pocket.to_account_info(),
                    coin_vault: self.coin_vault.to_account_info(),
                    pc_vault: self.pc_vault.to_account_info(),
                    coin_wallet: self.pocket_base_token_vault.to_account_info(),
                    pc_wallet: self.pocket_target_token_vault.to_account_info(),
                    vault_signer: self.pocket.to_account_info(),
                    token_program: self.token_program.to_account_info(),
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