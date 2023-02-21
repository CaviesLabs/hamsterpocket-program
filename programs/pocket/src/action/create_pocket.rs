use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct CreatePocketParams {
    // Id of the proposal
    pub id: String,

    // Define the name of the pool
    pub name: String,

    // Define the market key
    pub market_key: Pubkey,

    // Define base token
    pub base_token_address: Pubkey,

    // Define target token
    pub target_token_address: Pubkey,

    // Here we define the batch volume, the amount swap every batches
    pub batch_volume: u64,

    // Define the activated time the pool has settled
    pub start_at: u64,

    // Define the buy condition
    pub buy_condition: Option<BuyCondition>,

    // Define the stop condition
    pub stop_conditions: Vec<StopCondition>,

    // Define the frequency
    pub frequency: DateDuration,
}

#[derive(Accounts)]
#[instruction(params: CreatePocketParams)]
pub struct CreatePocketContext<'info> {
    #[account(
        init,
        seeds = [POCKET_SEED, params.id.as_bytes().as_ref()],
        payer = signer,
        space = 10240,
        bump
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [PLATFORM_SEED],
        bump = pocket_registry.bump,
    )]
    pub pocket_registry: Account<'info, PocketPlatformRegistry>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = spl_token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreatePocketContext<'info> {
    pub fn execute(&mut self, params: CreatePocketParams, pocket_bump: u8) -> Result<()> {
        // Update pocket state
        self.initialize_pocket(params, pocket_bump).unwrap();

        // Return instruction result
        Ok(())
    }

    fn initialize_pocket(&mut self, params: CreatePocketParams, pocket_bump: u8) -> Result<()> {
        // propagate data
        self.pocket.id = params.id;
        self.pocket.start_at = params.start_at;
        self.pocket.name = params.name;
        self.pocket.base_token_mint_address = params.base_token_address;
        self.pocket.target_token_mint_address = params.target_token_address;
        self.pocket.batch_volume = params.batch_volume;
        self.pocket.buy_condition = params.buy_condition;
        self.pocket.stop_conditions = params.stop_conditions;
        self.pocket.frequency = params.frequency;
        self.pocket.market_key = params.market_key;

        // assign default values
        self.pocket.bump = pocket_bump;
        self.pocket.owner = self.signer.key();
        self.pocket.status = PocketStatus::Active;

        // must check for valid data
        let pocket = self.pocket.clone();
        pocket.validate_pocket_data().unwrap();

        // emit event
        pocket_emit!(
          PocketCreated {
                pocket_address: pocket.key().clone(),
                market_key: pocket.market_key.clone(),
                owner: pocket.owner.clone(),
                name: pocket.name.clone()
            }
        );

        Ok(())
    }
}