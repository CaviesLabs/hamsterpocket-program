use crate::*;

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
    pub pocket_target_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_execute_swap<'info>(ctx: &Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>) -> Result<()> {
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

    // TODO: check for buy condition

    // Make Swap
    swap(&ctx).unwrap();

    Ok(())
}

fn swap<'info>(ctx: &Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>) -> Result<()> {
    let pocket = ctx.accounts.pocket.clone();

    let mut side = Side::Ask;

    if pocket.side == TradeSide::Buy {
        side = Side::Bid;
    }

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
                Side::Bid => ctx.accounts.pocket_target_token_vault.to_account_info(),
                Side::Ask => ctx.accounts.pocket_base_token_vault.to_account_info(),
            },
            coin_wallet: ctx.accounts.pocket_base_token_vault.to_account_info(),
            coin_vault: coin_vault.to_account_info(),
            pc_vault: pc_vault.to_account_info(),
            vault_signer: market_authority.to_account_info(),
        },
        authority: ctx.accounts.pocket.to_account_info(),
        pc_wallet: ctx.accounts.pocket_target_token_vault.to_account_info(),
        dex_program: dex_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        pocket: ctx.accounts.pocket.clone(),
    }, side, pocket.batch_volume, ExchangeRate {
        rate: 0,
        from_decimals: 0,
        quote_decimals: 0,
        strict: false,
    }).unwrap();

    Ok(())
}