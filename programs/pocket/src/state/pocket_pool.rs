use crate::*;

// Define the Price condition struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceCondition {
    Gt {
        value: u64
    },

    Gte {
        value: u64
    },

    Lt {
        value: u64
    },

    Lte {
        value: u64
    },

    Eq {
        value: u64
    },

    Neq {
        value: u64
    },

    Bw {
        from_value: u64,
        to_value: u64,
    },

    Nbw {
        from_value: u64,
        to_value: u64,
    },
}

impl PriceCondition {
    pub fn default() -> PriceCondition {
        PriceCondition::Eq { value: 0 }
    }

    pub fn is_valid(price: &PriceCondition) -> bool {
        return match price {
            PriceCondition::Gt { value } => {
                value.clone() > 0
            },

            PriceCondition::Gte { value } => {
                value.clone() > 0
            },

            PriceCondition::Lt { value } => {
                value.clone() > 0
            },

            PriceCondition::Lte { value } => {
                value.clone() > 0
            },

            PriceCondition::Eq { value } => {
                value.clone() > 0
            },

            PriceCondition::Neq { value } => {
                value.clone() > 0
            },

            PriceCondition::Bw { from_value, to_value } => {
                to_value.clone() >= from_value.clone() && from_value.clone() > 0
            },

            PriceCondition::Nbw { from_value, to_value } => {
                to_value.clone() >= from_value.clone() && from_value.clone() > 0
            },
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum StopCondition {
    EndTimeReach {
        is_primary: bool,
        value: u64
    },

    BaseTokenAmountReach {
        is_primary: bool,
        value: u64
    },

    TargetTokenAmountReach {
        is_primary: bool,
        value: u64
    },

    BatchAmountReach {
        is_primary: bool,
        value: u64
    },
}

impl StopCondition {
    pub fn default() -> StopCondition {
        StopCondition::EndTimeReach { is_primary: true, value: 0 }
    }

    // Check whether the stop condition is valid
    pub fn is_valid(stop_condition: &StopCondition) -> bool {
        return match stop_condition {
            StopCondition::EndTimeReach { value, .. } => {
                value.clone() > 0
            },
            StopCondition::BaseTokenAmountReach { value,  ..  } => {
                value.clone() > 0
            },
            StopCondition::TargetTokenAmountReach { value,  ..  } => {
                value.clone() > 0
            },
            StopCondition::BatchAmountReach { value,  ..  } => {
                value.clone() > 0
            }
        }
    }

    // Check whether the stop condition is primary
    pub fn is_primary(stop_condition: &StopCondition) -> bool {
        return match stop_condition {
            StopCondition::EndTimeReach { is_primary, .. } => {
                *is_primary == true
            },
            StopCondition::BaseTokenAmountReach { is_primary,  ..  } => {
                *is_primary == true
            },
            StopCondition::TargetTokenAmountReach { is_primary,  ..  } => {
                *is_primary == true
            },
            StopCondition::BatchAmountReach { is_primary,  ..  } => {
                *is_primary == true
            }
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub struct DateDuration {
    hours: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum TradeSide {
    #[default]
    Buy,
    Sell
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum MainProgressBy {
    #[default]
    EndTimeReach,
    BaseTokenAmountReach,
    TargetTokenAmountReach,
    BatchAmountReach
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
    pub base_token_mint_address: Pubkey,

    // Define target token
    pub target_token_mint_address: Pubkey,

    // Define the associated market
    pub market_key: Pubkey,

    // Here we define the batch volume, the amount swap every batches
    pub batch_volume: u64,

    // Define the activated time the pool has settled
    pub start_at: u64,

    // Define the trade side whether it is sell or buy
    pub side: TradeSide,

    // Define the buy condition
    pub buy_condition: Option<PriceCondition>,

    // Define the stop condition
    pub stop_conditions: Vec<StopCondition>,

    // Define the frequency
    pub frequency: DateDuration,

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

    // Check whether the pocket is able to close
    pub fn is_able_to_withdraw(&self) -> bool {
        return self.status == PocketStatus::Closed;
    }

    // Check whether the pocket is able to restart
    pub fn is_able_to_restart(&self) -> bool {
        return self.status == PocketStatus::Paused;
    }

    // Check whether the pocket is able to pause
    pub fn is_able_to_pause(&self) -> bool {
        return self.status == PocketStatus::Active;
    }

    // Check whether the pocket is able to swap
    pub fn is_ready_to_swap(&self) -> bool {
        return self.status == PocketStatus::Active && self.start_at >= Clock::get().unwrap().unix_timestamp as u64
    }

    // Check whether the pocket data is valid
    pub fn validate_pocket_data(&self) -> Result<()> {
        let pocket = self.clone();

        assert_ne!(pocket.name, String::default(), "Must initialize pocket name");
        assert_ne!(pocket.id, String::default(), "Pocket Id is not valid");

        assert_ne!(pocket.owner, Pubkey::default(), "Owner is not valid");
        assert_ne!(pocket.base_token_mint_address, Pubkey::default(), "Not valid pubkey");
        assert_ne!(pocket.target_token_mint_address, Pubkey::default(), "Not valid pubkey");

        assert_eq!(pocket.start_at >= Clock::get().unwrap().unix_timestamp as u64, true, "Not valid timestamp");
        assert_eq!(pocket.frequency.hours > 0, true, "Not valid frequency");
        assert_eq!(pocket.batch_volume > 0, true, "Not valid batch volume");

        if pocket.buy_condition.unwrap_or(PriceCondition::default()) != PriceCondition::default() {
            assert_eq!(PriceCondition::is_valid(&pocket.buy_condition.unwrap()), true, "Not valid buy condition");
        }

        let mut primary_count = 0;

        for x in pocket.stop_conditions {
            assert_eq!(StopCondition::is_valid(&x), true, "Not valid stop condition");
            if StopCondition::is_primary(&x) {
                primary_count = primary_count + 1;
            }
        }

        assert_eq!(primary_count <= 1, true, "Can just set 1 primary condition");

        Ok(())
    }
}