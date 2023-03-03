use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub enum DepositedTokenType {
    #[default]
    Base,
    Quote,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct DepositParams {
    pub mode: DepositedTokenType,
    pub deposit_amount: u64,
}

#[derive(Accounts)]
pub struct DepositContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: skip check
    #[account(mut)]
    pub signer_base_token_account: AccountInfo<'info>,

    /// CHECK: skip check
    #[account(mut)]
    pub signer_quote_token_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner
    )]
    pub pocket: Account<'info, Pocket>,

    /// CHECK: skip check
    #[account(
     mut,
    )]
    pub pocket_base_token_vault: AccountInfo<'info>,

    /// CHECK: skip check
    #[account(
        mut,
    )]
    pub pocket_quote_token_vault: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> DepositContext<'info> {
    pub fn execute(&mut self, params: DepositParams) -> Result<()> {
        let pocket = &mut self.pocket;

        assert_eq!(pocket.is_able_to_deposit(), true, "NOT_ABLE_TO_DEPOSIT");

        match params.mode {
            DepositedTokenType::Base => {
                // transfer the token
                token::transfer(
                    CpiContext::new(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.signer_base_token_account.to_account_info(),
                            to: self.pocket_base_token_vault.to_account_info(),
                            authority: self.signer.to_account_info(),
                        },
                    ),
                    params.deposit_amount.clone(),
                ).unwrap();

                // update credited balance
                pocket.base_token_balance += params.deposit_amount.clone();
                pocket.total_base_deposit_amount += params.deposit_amount.clone();
            }

            DepositedTokenType::Quote => {
                // transfer the token
                token::transfer(
                    CpiContext::new(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.signer_quote_token_account.to_account_info(),
                            to: self.pocket_quote_token_vault.to_account_info(),
                            authority: self.signer.to_account_info(),
                        },
                    ),
                    params.deposit_amount.clone(),
                ).unwrap();

                // update credited balance
                pocket.quote_token_balance += params.deposit_amount.clone();
                pocket.total_quote_deposit_amount += params.deposit_amount.clone();
            }
        }

        // emit event
        pocket_emit!(
            PocketDeposited {
               owner: self.signer.key(),
               pocket_address: pocket.key(),
               mint_address: match params.mode {
                    DepositedTokenType::Base => {
                        pocket.base_token_mint_address
                    }

                    DepositedTokenType::Quote => {
                        pocket.quote_token_mint_address
                    }
               },
               amount: params.deposit_amount.clone(),
            }
        );

        Ok(())
    }
}