//! Macros
pub use crate::*;

#[macro_export]
macro_rules! token_account_signer {
    ($seed: expr, $bump: expr) => {
        &[&[$seed, $bump][..]]
    };
}
