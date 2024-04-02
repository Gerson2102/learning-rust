use std::collections::BTreeMap;

/*
    TODO:
    Update the `Pallet` struct to be generic over the `AccountId` and `Balance` type.

    You won't need the type definitions above after you are done.
    Types will now be defined in `main.rs`. See the TODOs there.
*/
/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
/* TODO: Add the derive macro to implement the `Debug` trait for `Pallet`. */
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<AccountId, Balance>,
}

/*
    TODO:
    The generic types need to satisfy certain traits in order to be used in the functions below.
        - AccountId: Ord
        - Balance: Zero + CheckedSub + CheckedAdd + Copy

    You could figure these traits out yourself by letting the compiler tell you what you're missing.

    NOTE: You might need to adjust some of the functions below to satisfy the borrow checker.
*/
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::marker::Copy;

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy + std::cmp::PartialOrd,
{
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
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
    #[test]

    fn init_balances() {
        /* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = Pallet::<String, u128>::new();

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
        let mut balances = Pallet::<String, u128>::new();

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
