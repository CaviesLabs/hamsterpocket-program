use crate::*;

// Define params
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct UpdatePocketRegistryParams {
    pub operators: Vec<Pubkey>,
}

// Define the context, passed in parameters when trigger from deployer.
#[derive(Accounts)]
pub struct UpdatePocketRegistryContext<'info> {
    // We define the fee payer
    #[account(
        mut,
        address = pocket_registry.owner @ PocketError::OnlyAdministrator
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_SEED],
        bump = pocket_registry.bump,
        has_one = owner
    )]
    pub pocket_registry: Account<'info, PocketPlatformRegistry>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

// implement the handler
impl<'info> UpdatePocketRegistryContext<'info> {
    pub fn execute(&mut self, params: UpdatePocketRegistryParams) -> Result<()> {
        let pocket_registry = &mut self.pocket_registry;
        
        pocket_registry.operators = params.operators.clone();

        pocket_emit!(
            PocketConfigUpdated {
                actor: self.owner.key(),
                operators: params.operators.clone()
            }
        );

        Ok(())
    }
}
