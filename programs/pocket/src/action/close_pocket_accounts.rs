use crate::*;

#[derive(Accounts)]
pub struct ClosePocketAccountsContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pocket_quote_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pocket.owner == signer.key()
        && pocket.status == PocketStatus::Withdrawn @ PocketError::OnlyOwner,
        close = signer
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> ClosePocketAccountsContext<'info> {
    pub fn execute(&mut self) -> Result<()> {
        let pocket = &self.pocket;

        // find the bump to sign with the pda
        let bump = &[pocket.bump][..];
        let signer = &[&[POCKET_SEED, pocket.id.as_bytes().as_ref(), bump][..]];

        // close base token vault
        token::close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::CloseAccount {
                    account: self.pocket_base_token_vault.to_account_info().clone(),
                    destination: self.signer.to_account_info().clone(),
                    authority: pocket.to_account_info().clone(),
                },
                signer,
            )
        ).unwrap();

        // close base token vault
        token::close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::CloseAccount {
                    account: self.pocket_quote_token_vault.to_account_info().clone(),
                    destination: self.signer.to_account_info().clone(),
                    authority: pocket.to_account_info().clone(),
                },
                signer,
            )
        ).unwrap();

        // return result
        Ok(())
    }
}