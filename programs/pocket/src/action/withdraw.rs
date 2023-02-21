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
    pub signer_target_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pocket_target_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawContext<'info> {
    pub fn execute(&mut self) -> Result<()> {
        let pocket = &mut self.pocket;

        assert_eq!(pocket.is_able_to_withdraw(), true, "The pocket is not able to be withdrawn");

        let pocket_base_token_vault = &self.pocket_base_token_vault;
        let pocket_target_token_vault = &self.pocket_target_token_vault;
        
        let signer_base_token_vault = &self.signer_base_token_account;
        let signer_target_token_vault = &self.signer_target_token_account;

        // find the bump to sign with the pda
        let bump = &[pocket.bump][..];
        let signer =  &[&[POCKET_SEED, pocket.id.as_bytes().as_ref(), bump][..]];

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
                    from: pocket_target_token_vault.to_account_info(),
                    to: signer_target_token_vault.to_account_info(),
                    authority: pocket.to_account_info(),
                },
                signer,
            ),
            self.pocket_target_token_vault.amount,
        ).unwrap();

        // update credited balance
        pocket.base_token_balance = 0;
        pocket.target_token_balance = 0;

        Ok(())
    }
}