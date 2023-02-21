use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct DepositParams {
    pub base_token_vault_bump: u8,
    pub deposit_amount: u64
}

#[derive(Accounts)]
#[instruction(params: DepositParams)]
pub struct DepositContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    /// CHECK: the signer token account can be verified later
    pub signer_token_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [POCKET_SEED, pocket.id.as_bytes().as_ref()],
        bump = pocket.bump,
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(
        mut,
        seeds = [TOKEN_ACCOUNT_SEED, pocket.id.as_bytes().as_ref(), pocket.base_token_mint_address.key().as_ref()],
        bump = params.base_token_vault_bump
    )]
    pub pocket_base_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> DepositContext<'info> {
    pub fn execute(&mut self, params: DepositParams) -> Result<()> {
        let pocket = &mut self.pocket;

        assert_eq!(pocket.is_able_to_deposit(), true, "The pocket is not able to be deposited");

        // transfer the token
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.signer_token_account.to_account_info(),
                    to: self.pocket_base_token_vault.to_account_info(),
                    authority: self.signer.to_account_info(),
                },
            ),
            params.deposit_amount.clone(),
        ).unwrap();

        // update credited balance
        pocket.base_token_balance += params.deposit_amount.clone();
        pocket.total_deposit_amount += params.deposit_amount.clone();

        Ok(())
    }
}