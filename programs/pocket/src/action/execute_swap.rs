// use crate::*;
//
// #[derive(Accounts)]
// pub struct ExecuteSwapContext<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//
//     #[account(
//         mut,
//         constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner
//     )]
//     pub pocket: Account<'info, Pocket>,
//
//     #[account(mut)]
//     /// CHECK: the signer token account can be verified later
//     pub signer_token_account: AccountInfo<'info>,
//
//     #[account(
//         seeds = [PLATFORM_SEED],
//         bump = pocket_registry.bump,
//     )]
//     pub pocket_registry: Account<'info, PocketPlatformRegistry>,
//
//     #[account(mut)]
//     pub pocket_base_token_vault: Account<'info, TokenAccount>,
//
//     #[account(address = anchor_spl::dex::ID)]
//     pub dex_program: Program<'info, OpenBookDex>,
//
//     #[account(address = system_program::ID)]
//     pub system_program: Program<'info, System>,
//
//     #[account(address = spl_token::ID)]
//     pub token_program: Program<'info, Token>,
// }
//
// impl<'info> ExecuteSwapContext<'info> {
//     pub fn execute(&mut self) -> Result<()> {
//         let pocket_registry = self.pocket_registry.clone();
//         let pocket = self.pocket.clone();
//         let dex_program = self.dex_program.clone();
//         let signer = self.signer.clone();
//
//         // Only allow operator to perform the swap
//         if !pocket_registry.is_operator(signer.key()) {
//             return Err(PocketError::OnlyOperator.into());
//         }
//
//         // Check whether the pocket is ready to swap
//         if !pocket.is_ready_to_swap() {
//             return Err(PocketError::NotReadyToSwap.into());
//         }
//
//         if !pocket
//
//         Ok(())
//     }
//
//     fn check_for_buy_condition(&self) -> Result<()> {
//
//     }
//
//     fn place_swap(&mut self) -> Result<()> {
//         let pocket = self.pocket.clone();
//         let dex_program = self.dex_program.clone();
//
//         new_order_v3(
//             CpiContext::new_with_signer(
//                 dex_program.to_account_info(),
//                 NewOrderV3 {
//                     market: market.to_account_info(),
//                     coin_vault: coin_vault.to_account_info(),
//                     pc_vault: pc_vault.to_account_info(),
//                     request_queue: request_queue.to_account_info(),
//                     event_queue: event_queue.to_account_info(),
//                     market_bids: market_bids.to_account_info(),
//                     market_asks: market_asks.to_account_info(),
//                     open_orders: open_orders.to_account_info(),
//                     order_payer_token_account: dca_pc_vault.to_account_info(),
//                     open_orders_authority: dca.to_account_info(),
//                     token_program: token_program.to_account_info(),
//                     rent: rent.to_account_info(),
//                 },
//                 &[&[
//                     POCKET_SEED,
//                     pocket.id.as_bytes().as_ref(),
//                     &[pocket.bump],
//                 ]],
//             ),
//             Side::Bid,
//             NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
//             NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
//             NonZeroU64::new(dca.swap_amount).unwrap(),
//             SelfTradeBehavior::DecrementTake,
//             OrderType::Limit,
//             u64::try_from_slice(&dca.key().to_bytes()[0..8]).unwrap(),
//             std::u16::MAX,
//         )?;
//
//         settle_funds(
//             CpiContext::new_with_signer(
//                 dex_program.to_account_info(),
//                 SettleFunds {
//                     market: market.to_account_info(),
//                     open_orders: open_orders.to_account_info(),
//                     open_orders_authority: dca.to_account_info(),
//                     coin_vault: coin_vault.to_account_info(),
//                     pc_vault: pc_vault.to_account_info(),
//                     coin_wallet: dca_coin_vault.to_account_info(),
//                     pc_wallet: dca_pc_vault.to_account_info(),
//                     vault_signer: vault_signer.to_account_info(),
//                     token_program: token_program.to_account_info(),
//                 },
//                 &[&[
//                     POCKET_SEED,
//                     pocket.id.as_bytes().as_ref(),
//                     &[pocket.bump],
//                 ]],
//             ))?;
//
//         Ok(())
//     }
// }