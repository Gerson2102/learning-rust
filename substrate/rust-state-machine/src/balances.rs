use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
use std::marker::Copy;

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + std::cmp::PartialOrd;
}
/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
/* TODO: Add the derive macro to implement the `Debug` trait for `Pallet`. */
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        /* TODO:
            - Get the balance of account `caller`.
            - Get the balance of account `to`.

            - Use safe math to calculate a `new_caller_balance`.
            - Use safe math to calculate a `new_to_balance`.

            - Insert the new balance of `caller`.
            - Insert the new balance of `to`.
        */

        // - Get the balance of account `caller`.
        let caller_balance = self.balance(&caller);
        //- Get the balance of account `to`.
        let to_balance = self.balance(&to);

        if caller_balance < amount {
            return Err("Insufficient balance for transfer");
        }

        // - Use safe math to calculate a `new_caller_balance`.
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Underflow because of substraction!")?;
        //- Use safe math to calculate a `new_to_balance`.
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow because of addition!")?;
        // - Insert the new balance of `caller`.
        self.balances.insert(caller, new_caller_balance);
        // - Insert the new balance of `to`.
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    pub use crate::balances::Pallet;

    pub struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        /* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = Pallet::<TestConfig>::new();

        /* TODO: Assert that the balance of `alice` starts at zero. */
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        /* TODO: Set the balance of `alice` to 100. */
        balances.set_balance(&"alice".to_string(), 100);
        /* TODO: Assert the balance of `alice` is now 100. */
        assert_eq!(balances.balance(&"bob".to_string()), 0);
        /* TODO: Assert the balance of `bob` has not changed and is 0. */
        assert_eq!(balances.balance(&"alice".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        /* TODO: Create a test that checks the following:
            - That `alice` cannot transfer funds she does not have.
            - That `alice` can successfully transfer funds to `bob`.
            - That the balance of `alice` and `bob` is correctly updated.
        */
        /*
            TODO:
            When creating an instance of `Pallet`, you should explicitly define the types you use.
        */
        let mut balances = Pallet::<TestConfig>::new();

        balances.set_balance(&"alice".to_string(), 50);
        let result = balances.transfer("alice".to_string(), "bob".to_string(), 100);
        assert_eq!(result, Err("Insufficient balance for transfer"));
        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 0);

        let result2 = balances.transfer("alice".to_string(), "bob".to_string(), 25);
        assert!(result2.is_ok());
        assert_eq!(balances.balance(&"alice".to_string()), 25);
        assert_eq!(balances.balance(&"bob".to_string()), 25);
    }
}
