mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::{Dispatch, DispatchResult};

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    use crate::support;
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
/* TODO: Add the derive macro to implement the `Debug` trait for `Runtime`. */
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
    /* TODO: Move your type definitions for `BlockNumber` and `Nonce` here. */
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: (bob),
                    amount: (20),
                }),
            },
            support::Extrinsic {
                caller: "alice".to_string().clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: (charlie),
                    amount: (20),
                }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: (&"HELLO!"),
                }),
            },
            support::Extrinsic {
                caller: "bob".to_string().clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: (&"HELLO!"),
                }),
            },
        ],
    };

    /*
        TODO:
        Create new block(s) which execute extrinsics for the new `ProofOfExistence` pallet.
            - Make sure to set the block number correctly.
            - Feel free to allow some extrinsics to fail, and see the errors appear.
    */

    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");

    println!("{:?}", runtime);
}
