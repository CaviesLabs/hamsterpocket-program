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
    make_swap(&ctx).unwrap();

    Ok(())
}

fn make_swap<'info>(ctx: &Context<'_, '_, '_, 'info, ExecuteSwapContext<'info>>) -> Result<()> {
    let pocket = ctx.accounts.pocket.clone();

    let mut side = Side::Bid;

    if pocket.side == TradeSide::Sell {
        side = Side::Ask;
    }

    let market = &mut ctx.remaining_accounts.get(0).unwrap();
    let event_queue = &mut ctx.remaining_accounts.get(1).unwrap();
    let request_queue = &mut ctx.remaining_accounts.get(2).unwrap();
    let market_bids = &mut ctx.remaining_accounts.get(3).unwrap();
    let market_asks = &mut ctx.remaining_accounts.get(4).unwrap();
    let coin_vault = &mut ctx.remaining_accounts.get(5).unwrap();
    let pc_vault = &mut ctx.remaining_accounts.get(6).unwrap();
    let market_authority = &mut ctx.remaining_accounts.get(7).unwrap();
    let open_orders = &mut ctx.remaining_accounts.get(8).unwrap();
    let dex_program = &mut ctx.remaining_accounts.get(9).unwrap();

    new_order_v3(
        CpiContext::new_with_signer(
            dex_program.to_account_info(),
            NewOrderV3 {
                market: market.to_account_info(),
                coin_vault: coin_vault.to_account_info(),
                pc_vault: pc_vault.to_account_info(),
                request_queue: request_queue.to_account_info(),
                event_queue: event_queue.to_account_info(),
                market_bids: market_bids.to_account_info(),
                market_asks: market_asks.to_account_info(),
                open_orders: open_orders.to_account_info(),
                order_payer_token_account: ctx.accounts.pocket_base_token_vault.to_account_info(),
                open_orders_authority: ctx.accounts.pocket.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
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
                market: market.to_account_info(),
                open_orders: open_orders.to_account_info(),
                coin_vault: coin_vault.to_account_info(),
                pc_vault: pc_vault.to_account_info(),
                vault_signer: market_authority.to_account_info(),
                open_orders_authority: ctx.accounts.pocket.to_account_info(),
                coin_wallet: ctx.accounts.pocket_base_token_vault.to_account_info(),
                pc_wallet: ctx.accounts.pocket_target_token_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
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