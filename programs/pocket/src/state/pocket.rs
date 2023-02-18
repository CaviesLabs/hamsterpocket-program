use crate::*;

// Define the Price condition struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceCondition {
    GT {
        value: u64
    },

    GTE {
        value: u64
    },

    LT {
        value: u64
    },

    LTE {
        value: u64
    },

    EQ {
        value: u64
    },

    NEQ {
        value: u64
    },

    BW {
        from_value: u64,
        to_value: u64,
    },

    NBW {
        from_value: u64,
        to_value: u64,
    },
}

impl PriceCondition {
    pub fn default() -> PriceCondition {
        PriceCondition::EQ { value: 0 }
    }
}

// Define the BuyCondition struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct BuyCondition {
    pub token_address: Pubkey,
    pub condition: PriceCondition,
}

impl BuyCondition {
    pub fn default() -> BuyCondition {
        BuyCondition {
            token_address: Pubkey::default(),
            condition: PriceCondition::default(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum StopCondition {
    EndTime {
        value: u64
    },

    BaseTokenReach {
        value: u64
    },

    TargetTokenReach {
        value: u64
    },

    BatchAmountReach {
        value: u64
    },
}

impl StopCondition {
    pub fn default() -> StopCondition {
        StopCondition::EndTime { value: 0 }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub struct DateDuration {
    hours: u64,
}

// ================ Pocket Option Interface ================ //
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum PocketStatus {
    // Declare that the Pocket is created
    #[default]
    Active,

    // Declare that the Pocket is paused
    Paused,

    // Declare that the Pocket is Closed
    Closed,

    // Declare that the proposal is fully withdrawn by both participant and proposal owner
    Withdrawn,
}

// Here we define the account state that holds the pocket order. Pocket will be the PDA.
#[account]
#[derive(Default)]
pub struct Pocket {
    // Id of the proposal
    pub id: String,

    // Bump to help define the PDA of pocket order.
    pub bump: u8,

    // Define the owner of the proposal
    pub owner: Pubkey,

    // Define the name of the pool
    pub name: String,

    // Define the proposal status
    pub status: PocketStatus,

    // Define base token
    pub base_token_address: Pubkey,

    // Define target token
    pub target_token_address: Pubkey,

    // Define the market key
    pub market_key: Pubkey,

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

    ////// Define progress fields ////////
    // Show total deposited base token balance
    pub total_deposit_amount: u64,

    // Show base token balance
    pub base_token_balance: u64,

    // Show target token balance
    pub target_token_balance: u64,

    // Show batch amount
    pub executed_batch_amount: u64,

    // Next schedule date
    pub next_scheduled_execution_at: u64,
}

impl Pocket {
    // Check whether the pocket is open for depositing
    pub fn is_able_to_deposit(&self) -> bool {
        return self.status != PocketStatus::Closed && self.status != PocketStatus::Withdrawn;
    }

    // Check whether the pocket is able to close
    pub fn is_able_to_close(&self) -> bool {
        return self.status != PocketStatus::Closed && self.status != PocketStatus::Withdrawn;
    }

    // Check whether the pocket is able to restart
    pub fn is_able_to_restart(&self) -> bool {
        return self.status == PocketStatus::Paused;
    }

    // Check whether the pocket is able to pause
    pub fn is_able_to_pause(&self) -> bool {
        return self.status == PocketStatus::Active;
    }

    pub fn is_ready_to_swap(&self) -> bool {
        return self.status == PocketStatus::Active && self.start_at >= Clock::get().unwrap().unix_timestamp as u64
    }
}