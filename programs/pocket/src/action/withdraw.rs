use crate::*;

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(mut)]
    /// CHECK: the signer token account can be verified later
    pub signer_base_token_account: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: the signer token account can be verified later
    pub signer_quote_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pocket_quote_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawContext<'info> {
    pub fn execute(&mut self) -> Result<()> {
        let pocket = &mut self.pocket;

        assert_eq!(pocket.is_able_to_withdraw(), true, "NOT_ABLE_TO_WITHDRAW");

        let pocket_base_token_vault = &self.pocket_base_token_vault;
        let pocket_quote_token_vault = &self.pocket_quote_token_vault;

        let signer_base_token_vault = &self.signer_base_token_account;
        let signer_quote_token_vault = &self.signer_quote_token_account;

        // update credited balance & status
        pocket.base_token_balance = 0;
        pocket.quote_token_balance = 0;
        pocket.status = PocketStatus::Withdrawn;

        // find the bump to sign with the pda
        let bump = &[pocket.bump][..];
        let signer = &[&[POCKET_SEED, pocket.id.as_bytes().as_ref(), bump][..]];

        // transfer the token
        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: pocket_base_token_vault.to_account_info(),
                    to: signer_base_token_vault.to_account_info(),
                    authority: pocket.to_account_info(),
                },
                signer,
            ),
            self.pocket_base_token_vault.amount,
        ).unwrap();

        // transfer the token
        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: pocket_quote_token_vault.to_account_info(),
                    to: signer_quote_token_vault.to_account_info(),
                    authority: pocket.to_account_info(),
                },
                signer,
            ),
            self.pocket_quote_token_vault.amount,
        ).unwrap();

        // emit event
        pocket_emit!(
            PocketWithdrawn {
               owner: self.signer.key(),
               pocket_address: pocket.key(),
               base_token_mint_address: pocket.base_token_mint_address,
               base_token_amount: self.pocket_base_token_vault.amount,
               quote_token_mint_address: pocket.quote_token_mint_address,
               quote_token_amount: self.pocket_quote_token_vault.amount
            }
        );

        Ok(())
    }
}