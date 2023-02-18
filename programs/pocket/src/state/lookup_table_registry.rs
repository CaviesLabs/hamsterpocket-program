use crate::*;

// Here we define the account state that holds the lookup table for every users.
#[account]
#[derive(Default)]
pub struct LookupTableRegistry {
    // Bump to help define the PDA of pocket order.
    pub bump: u8,

    // Define the owner of the proposal
    pub owner: Pubkey,

    // Define the lookup table for specific users
    pub lookup_table_addresses: Vec<Pubkey>,
}