use crate::*;

// Define params
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct InitializePocketPlatformParams {
    pub operators: Vec<Pubkey>,
}

// Define the context, passed in parameters when trigger from deployer.
#[derive(Accounts)]
pub struct InitializePocketPlatformContext<'info> {
    // We define the fee payer
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [PLATFORM_SEED],
        payer = owner,
        space = 10240,
        bump
    )]
    pub pocket_registry: Account<'info, PocketPlatformRegistry>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

// implement the handler
impl<'info> InitializePocketPlatformContext<'info> {
    pub fn execute(&mut self, params: InitializePocketPlatformParams, bump: u8) -> Result<()> {
        // Handle post initialization
        self.pocket_registry.handle_post_initialized().unwrap();

        // Assigning values
        let pocket_registry = &mut self.pocket_registry;
        pocket_registry.bump = bump;
        pocket_registry.owner = *self.owner.key;
        pocket_registry.operators = params.operators;

        Ok(())
    }
}
