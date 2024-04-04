use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    /* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        /* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            claims: BTreeMap::new(),
        }
    }
    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        /* TODO: `get` the `claim` */
        self.claims.get(claim)
    }

    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        /* TODO: Check that a `claim` does not already exist. If so, return an error. */
        /* TODO: `insert` the claim on behalf of `caller`. */
        if self.claims.contains_key(&claim) {
            return Err(&"Claim already exists!");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        /* TODO: Get the owner of the `claim` to be revoked. */
        let owner_of_claim = self.get_claim(&claim).ok_or("claim does not exists!")?;
        /* TODO: Check that the `owner` matches the `caller`. */
        if *owner_of_claim != caller {
            return Err(&"You can't revoke this claim, you are not the owner!");
        }
        /* TODO: If all checks pass, then `remove` the `claim`. */
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        /*
            TODO:
            Create an end to end test verifying the basic functionality of this pallet.
                - Check the initial state is as you expect.
                - Check that all functions work successfully.
                - Check that all error conditions error as expected.
        */
        let mut claim = Pallet::<TestConfig>::new();
        assert_eq!(claim.get_claim(&"Hello!"), None);
        assert_eq!(claim.create_claim(&"gerson", &"Hello PoE"), Ok(()));
        assert_eq!(claim.get_claim(&"Hello PoE"), Some(&"gerson"));
        assert_eq!(
            claim.create_claim(&"alice", &"Hello PoE"),
            Err("Claim already exists!")
        );
        assert_eq!(
            claim.revoke_claim(&"bob", &"Hello PoE"),
            Err("You can't revoke this claim, you are not the owner!")
        );
        assert_eq!(claim.revoke_claim(&"gerson", &"Hello PoE"), Ok(()));
    }
}
