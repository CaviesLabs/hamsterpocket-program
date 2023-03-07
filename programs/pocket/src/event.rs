//! Events emitted.
use crate::*;

// Log to Program Log with a prologue so transaction scraper knows following line is valid mango log
#[macro_export]
macro_rules! pocket_emit {
    ($e:expr) => {
        msg!("pocket-log");
        emit!($e);
    };
}

/// Emitted when a [PocketConfigUpdated] is created.
#[event]
pub struct PocketConfigUpdated {
    #[index]
    pub actor: Pubkey,
    pub operators: Vec<Pubkey>,
}

/// Emitted when a [VaultCreated] is created.
#[event]
pub struct VaultCreated {
    #[index]
    pub actor: Pubkey,
    #[index]
    pub authority: Pubkey,
    #[index]
    pub mint_account: Pubkey,
    #[index]
    pub associated_account: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
}

/// Emitted when a [PocketCreated] is created.
#[event]
pub struct PocketCreated {
    #[index]
    pub owner: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
    pub name: String,
}

/// Emitted when a [PocketUpdated] is created.
#[event]
pub struct PocketUpdated {
    #[index]
    pub actor: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
    pub status: PocketStatus,
    pub memo: String
}

/// Emitted when a [PocketDeposited] is created.
#[event]
pub struct PocketDeposited {
    #[index]
    pub owner: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
    #[index]
    pub mint_address: Pubkey,
    pub amount: u64
}

/// Emitted when a [PocketWithdrawn] is created.
#[event]
pub struct PocketWithdrawn {
    #[index]
    pub owner: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
    #[index]
    pub base_token_mint_address: Pubkey,
    pub base_token_amount: u64,
    #[index]
    pub quote_token_mint_address: Pubkey,
    pub quote_token_amount: u64
}