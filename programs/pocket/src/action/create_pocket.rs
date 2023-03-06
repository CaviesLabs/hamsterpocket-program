use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct CreatePocketParams {
    // Id of the proposal
    pub id: String,

    // Define the name of the pool
    pub name: String,

    // Define base token
    pub base_token_address: Pubkey,

    // Define target token
    pub quote_token_address: Pubkey,

    pub market_key: Pubkey,

    // Here we define the batch volume, the amount swap every batches
    pub batch_volume: u64,

    // Define the activated time the pool has settled
    pub start_at: u64,

    // Define the buy condition
    pub buy_condition: Option<PriceCondition>,

    // Define the stop condition
    pub stop_conditions: Vec<StopCondition>,

    // Define the frequency
    pub frequency: DateDuration,

    // Define the trade side
    pub side: TradeSide
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
        self.pocket.quote_token_mint_address = params.quote_token_address;
        self.pocket.batch_volume = params.batch_volume;
        self.pocket.buy_condition = params.buy_condition;
        self.pocket.stop_conditions = params.stop_conditions;
        self.pocket.frequency = params.frequency;
        self.pocket.side = params.side;
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
                owner: pocket.owner.clone(),
                name: pocket.name.clone()
            }
        );

        Ok(())
    }
}