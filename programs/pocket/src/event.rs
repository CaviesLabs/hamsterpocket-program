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
    pub max_allowed_items: u8,
    pub max_allowed_options: u8,
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
}

/// Emitted when a [PocketCreated] is created.
#[event]
pub struct PocketCreated {
    #[index]
    pub owner: Pubkey,
    #[index]
    pub pocket_address: Pubkey,
    #[index]
    pub market_key: Pubkey,
    pub name: String,
}