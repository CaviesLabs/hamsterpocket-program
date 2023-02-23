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
    #[msg("Only Platform operator")]
    OnlyOperator,

    #[msg("Only Platform Administrator")]
    OnlyAdministrator,

    #[msg("Only Owner")]
    OnlyOwner,

    #[msg("Not ready to swap")]
    NotReadyToSwap
}
