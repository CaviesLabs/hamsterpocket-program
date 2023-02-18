use crate::*;

#[error_code]
pub enum PocketError {
    // System error
    #[msg("System error")]
    SystemError,
    #[msg("The program was already initialized")]
    AlreadyInitialized,
    #[msg("The mint account was existed")]
    MintAccountExisted,
    // Business errors
    #[msg("Only Platform Admin")]
    OnlyAdministrator,
    #[msg("Only Owner")]
    OnlyOwner,
    #[msg("Only Buyer")]
    OnlyBuyer,
    #[msg("Only Seller")]
    OnlySeller,
    #[msg("Order expired")]
    OrderExpired,
    #[msg("Invalid Offer")]
    InvalidOffer,
    #[msg("Invalid value")]
    InvalidValue,
    #[msg("Invalid value")]
    UnAllowedMintToken,
    #[msg("Proposal cannot be canceled")]
    ProposalCannotBeCanceled,
    #[msg("Withdrawal is not available for the proposal")]
    WithdrawalIsNotAvailable,
    #[msg("Redeem is not available for the proposal")]
    RedeemIsNotAvailable,
    #[msg("Transfer token from vault is not available for the proposal")]
    TransferTokenFromVaultIsNotAvailable,
    #[msg("Deposit is not available for the proposal")]
    DepositIsNotAvailable,
    #[msg("Fulfilling is not available for the proposal")]
    FulfillingIsNotAvailable,
    #[msg("Only participants can execute this operation")]
    OnlyParticipant,
}
