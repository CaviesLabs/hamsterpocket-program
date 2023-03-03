use crate::*;

pub fn handle_execute_swap<'info>(ctx: Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>) -> Result<()> {
    let pocket_registry = ctx.accounts.pocket_registry.clone();
    let pocket = ctx.accounts.pocket.clone();
    let signer = ctx.accounts.signer.clone();

    // Only allow operator to perform the swap
    if !pocket_registry.is_operator(signer.key()) {
        return Err(PocketError::OnlyOperator.into());
    }

    // Check whether the pocket is ready to swap
    if !pocket.is_ready_to_swap() {
        return Err(PocketError::NotReadyToSwap.into());
    }

    // Make Swap
    let did_swap = swap(&ctx).unwrap();

    // Pocket risk check and update
    ctx.accounts.ensure_pocket_integrity(&did_swap).unwrap();

    // Return result
    Ok(())
}

fn swap<'info>(ctx: &Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>) -> Result<DidSwap> {
    let pocket = &ctx.accounts.pocket;

    // Determine side
    let mut side = Side::Ask;
    if pocket.side == TradeSide::Buy {
        side = Side::Bid;
    }

    let amount_to_swap = match pocket.side {
        TradeSide::Buy => {
            if pocket.batch_volume <= pocket.quote_token_balance {
                pocket.batch_volume
            } else {
                pocket.quote_token_balance
            }
        }

        TradeSide::Sell => {
            if pocket.batch_volume <= pocket.base_token_balance {
                pocket.batch_volume
            } else {
                pocket.base_token_balance
            }
        }
    };

    // Extract accounts
    let event_queue = &mut ctx.remaining_accounts.get(0).unwrap();
    let request_queue = &mut ctx.remaining_accounts.get(1).unwrap();
    let market_bids = &mut ctx.remaining_accounts.get(2).unwrap();
    let market_asks = &mut ctx.remaining_accounts.get(3).unwrap();
    let coin_vault = &mut ctx.remaining_accounts.get(4).unwrap();
    let pc_vault = &mut ctx.remaining_accounts.get(5).unwrap();
    let market_authority = &mut ctx.remaining_accounts.get(6).unwrap();
    let open_orders = &mut ctx.remaining_accounts.get(7).unwrap();
    let dex_program = &mut ctx.remaining_accounts.get(8).unwrap();

    // Make swap
    let did_swap = external::swap(Swap {
        market: MarketAccounts {
            market: ctx.accounts.market_key.to_account_info(),
            open_orders: open_orders.to_account_info(),
            request_queue: request_queue.to_account_info(),
            event_queue: event_queue.to_account_info(),
            bids: market_bids.to_account_info(),
            asks: market_asks.to_account_info(),
            order_payer_token_account: match side {
                Side::Bid => ctx.accounts.pocket_quote_token_vault.to_account_info(),
                Side::Ask => ctx.accounts.pocket_base_token_vault.to_account_info(),
            },
            coin_wallet: ctx.accounts.pocket_base_token_vault.to_account_info(),
            coin_vault: coin_vault.to_account_info(),
            pc_vault: pc_vault.to_account_info(),
            vault_signer: market_authority.to_account_info(),
        },
        authority: ctx.accounts.pocket.to_account_info(),
        pc_wallet: ctx.accounts.pocket_quote_token_vault.to_account_info(),
        dex_program: dex_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        pocket: ctx.accounts.pocket.clone(),
    }, side, amount_to_swap, ExchangeRate {
        rate: 0,
        from_decimals: 0,
        quote_decimals: 0,
        strict: false,
    }).unwrap();

    // Return
    Ok(did_swap)
}

#[derive(Accounts)]
pub struct ExecuteSwapContext<'info> {
    // Pocket accounts
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pocket: Account<'info, Pocket>,

    /// CHECK: skip verification
    #[account(
        mut,
        address = pocket.market_key,
    )]
    pub market_key: AccountInfo<'info>,

    #[account(
        seeds = [PLATFORM_SEED],
        bump = pocket_registry.bump,
    )]
    pub pocket_registry: Account<'info, PocketPlatformRegistry>,

    #[account(mut)]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pocket_quote_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> ExecuteSwapContext<'info> {
    pub fn ensure_pocket_integrity(&mut self, did_swap: &DidSwap) -> Result<()> {
        // Validate if the swap matched price condition
        self.check_for_swap_possibility(did_swap).unwrap();

        // Update pocket balance
        self.update_pocket_info(did_swap).unwrap();

        // Update Pocket status if matches stop condition
        self.update_pocket_status().unwrap();

        Ok(())
    }

    fn update_pocket_info(&mut self, swap_data: &DidSwap) -> Result<()> {
        let did_swap = swap_data.clone();
        let pocket = &mut self.pocket;

        // Update pocket balance
        match pocket.side {
            TradeSide::Buy => {
                pocket.base_token_balance = pocket.base_token_balance + did_swap.to_amount;
                pocket.quote_token_balance = pocket.quote_token_balance - did_swap.from_amount;
            }

            TradeSide::Sell => {
                pocket.base_token_balance = pocket.base_token_balance - did_swap.from_amount;
                pocket.quote_token_balance = pocket.quote_token_balance + did_swap.to_amount;
            }
        }

        // Update pocket info
        pocket.next_scheduled_execution_at = Clock::get().unwrap().unix_timestamp as u64 + pocket.frequency.hours.saturating_mul(3600);
        pocket.executed_batch_amount = pocket.executed_batch_amount + 1;

        Ok(())
    }

    fn update_pocket_status(&mut self) -> Result<()> {
        let mut should_stop = false;
        let pocket = &mut self.pocket;

        for condition in pocket.stop_conditions.clone() {
            match condition {
                StopCondition::EndTimeReach { value, .. } => {
                    if value <= Clock::get().unwrap().unix_timestamp as u64 {
                        should_stop = true;
                        break;
                    }
                }

                StopCondition::BaseTokenAmountReach { value,.. } => {
                    if value <= pocket.base_token_balance {
                        should_stop = true;
                        break;
                    }
                }

                StopCondition::QuoteTokenAmountReach { value, .. } => {
                    if value <= pocket.quote_token_balance {
                        should_stop = true;
                        break;
                    }
                }

                StopCondition::SpentBaseTokenAmountReach { value, .. } => {
                    if value <= pocket.total_base_deposit_amount.saturating_sub(pocket.base_token_balance) {
                        should_stop = true;
                        break;
                    }
                }

                StopCondition::SpentQuoteTokenAmountReach { value, .. } => {
                    if value <= pocket.total_quote_deposit_amount.saturating_sub(pocket.quote_token_balance) {
                        should_stop = true;
                        break;
                    }
                }

                StopCondition::BatchAmountReach { value, .. } => {
                    if value <= pocket.executed_batch_amount {
                        should_stop = true;
                        break;
                    }
                }
            }
        }

        // Force close pocket
        if should_stop {
            pocket.status = PocketStatus::Closed;
        }

        // Emit event
        pocket_emit!(PocketUpdated {
            actor: self.signer.key(),
            pocket_address: pocket.key(),
            status: pocket.status,
            memo: String::from("STOP_CONDITION_REACHED")
        });

        Ok(())
    }

    fn check_for_swap_possibility(&self, swap_data: &DidSwap) -> Result<()> {
        let did_swap = swap_data.clone();
        let pocket = &self.pocket;

        // Must match the next scheduled date and start date
        assert_eq!(
            pocket.is_ready_to_swap(),
            true,
            "NOT_READY_TO_SWAP"
        );

        // Check for buy condition
        match pocket.buy_condition.clone() {
            None => {}

            Some(PriceCondition::Bw { from_value, to_value }) => {
                assert_eq!(
                    did_swap.to_amount <= to_value && did_swap.to_amount >= from_value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Nbw { from_value, to_value }) => {
                assert_eq!(
                    did_swap.to_amount <= to_value && did_swap.to_amount >= from_value,
                    false,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Gt { value }) => {
                assert_eq!(
                    did_swap.to_amount > value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Gte { value }) => {
                assert_eq!(
                    did_swap.to_amount >= value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Lt { value }) => {
                assert_eq!(
                    did_swap.to_amount < value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Lte { value }) => {
                assert_eq!(
                    did_swap.to_amount <= value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Eq { value }) => {
                assert_eq!(
                    did_swap.to_amount == value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }

            Some(PriceCondition::Neq { value }) => {
                assert_eq!(
                    did_swap.to_amount != value,
                    true,
                    "BUY_CONDITION_NOT_FULFILLED"
                );
            }
        }

        Ok(())
    }
}

