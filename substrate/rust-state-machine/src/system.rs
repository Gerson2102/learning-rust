/* TODO: You might need to update your imports. */
use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
/* TODO: Add the derive macro to implement the `Debug` trait for `Pallet`. */
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    /* TODO: Create a field `block_number` that stores a `u32`. */
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    /* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        /* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        return self.block_number;
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* TODO: Increment the current block number by one. */
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    pub use crate::system::Pallet;
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    #[test]
    fn init_system() {
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.
            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
        */
        let mut system = Pallet::<TestConfig>::new();
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number, 1);
        assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
        assert_eq!(system.nonce.get(&"bob".to_string()), None);
    }
}
