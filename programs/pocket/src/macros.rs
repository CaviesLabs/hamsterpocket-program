//! Macros
pub use crate::*;

#[macro_export]
macro_rules! token_account_signer {
    ($pre_seed: expr, $seed: expr, $bump: expr) => {
        &[&[$pre_seed, $seed, $bump][..]]
    };
}
