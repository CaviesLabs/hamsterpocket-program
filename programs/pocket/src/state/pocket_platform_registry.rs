use crate::*;

// ================ Pocket Platform Config ================ //
// Here we define the account state that holds the administration info.
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, PartialEq)]
pub struct MintInfo {
    // Whether the mint token is active or not.
    pub is_enabled: bool,
    pub mint_account: Pubkey,
    pub token_account: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct PocketPlatformRegistry {
    // Define owner
    pub owner: Pubkey,

    // define whether the config was initialized or not, the contract must be only initialized once.
    pub was_initialized: bool,

    // Bump to help define the PDA of pocket account
    pub bump: u8,

    // define whitelisted mint token account
    pub allowed_mint_accounts: Vec<MintInfo>,

    // define whitelisted mint token account
    pub operators: Vec<Pubkey>,
}

// Define handler
impl PocketPlatformRegistry {
    // handle data integrity after initialization
    pub fn handle_post_initialized(&mut self) -> Result<()> {
        if self.was_initialized == false {
            self.was_initialized = true;
            return Ok(());
        }

        msg!("ERROR::PLATFORM::ALREADY_INITIALIZED");
        return Err(PocketError::AlreadyInitialized.into());
    }

    // Check whether the mint account was previously added or not.
    pub fn is_mint_account_existed(&self, mint_account: Pubkey) -> bool {
        return self.allowed_mint_accounts.iter()
            .map(|allowed_mint_account| allowed_mint_account.mint_account)
            .filter(|&mint_account_key| mint_account_key == mint_account.key().clone())
            .count() >= 1;
    }

    // Check whether the mint account was enabled or not
    pub fn is_mint_account_enabled(&self, mint_account: Pubkey) -> bool {
        return self.allowed_mint_accounts.iter()
            .filter(|&mint_info|
                mint_info.mint_account == mint_account.key().clone()
                    && mint_info.is_enabled == true
            )
            .count() >= 1;
    }

    // Get mint info
    pub fn get_mint_info(&self, mint_account: Pubkey) -> &MintInfo {
        return &self.allowed_mint_accounts.iter()
            .find(|&mint_account_key| mint_account_key.mint_account == mint_account.key().clone())
            .unwrap();
    }

    // Detect if a pubkey was belong to an operator
    pub fn is_operator(&self, operator_pubkey: Pubkey) -> bool {
        return self.operators.iter()
            .find(|&pubkey| pubkey.clone() == operator_pubkey.clone())
            .is_some();
    }
}