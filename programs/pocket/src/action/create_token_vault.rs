use crate::*;

#[derive(Accounts)]
pub struct CreateTokenVaultContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [POCKET_SEED, pocket.id.as_bytes().as_ref()],
        constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner,
        bump = pocket.bump,
    )]
    pub pocket: Account<'info, Pocket>,

    pub mint_account: Account<'info, Mint>,

    #[account(init,
        token::mint = mint_account,
        token::authority = pocket,
        seeds = [TOKEN_ACCOUNT_SEED, pocket.key().as_ref(), mint_account.key().as_ref()],
        payer = signer,
        bump
    )]
    pub pocket_token_vault: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateTokenVaultContext<'info> {
    pub fn execute(&mut self) -> Result<()> {
        // emit event
        pocket_emit!(
            VaultCreated {
                actor: self.signer.key().clone(),
                authority: self.pocket.key().clone(),
                associated_account: self.pocket_token_vault.key().clone(),
                mint_account: self.mint_account.key().clone(),
                pocket_address: self.pocket.key().clone()
            }
        );

        Ok(())
    }
}