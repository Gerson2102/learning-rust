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

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
/* TODO: Add the derive macro to implement the `Debug` trait for `Runtime`. */
#[derive(Debug)]
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

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        /* TODO:
            - Increment the system's block number.
            - Check that the block number of the incoming block matches the current block number,
              or return an error.
            - Iterate over the extrinsics in the block...
                - Increment the nonce of the caller.
                - Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
                - Handle errors from `dispatch` same as we did for individual calls: printing any
                  error and capturing the result.
                - You can extend the error message to include information like the block number and
                  extrinsic number.
        */
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err(&"block number does not match what is expected");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }
        Ok(())
    }
}

impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> DispatchResult {
        /*
            TODO:
            Use a match statement to route the `runtime_call` to call the appropriate function in
            our pallet. In this case, there is only `self.balances.transfer`.

            Your `runtime_call` won't contain the caller information which is needed to make the
            `transfer` call, but you have that information from the arguments to the `dispatch`
            function.

            You should propagate any errors from the call back up this function.
        */
        // This match statement will allow us to correctly route `RuntimeCall`s
        // to the appropriate pallet level function.
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
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
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: (bob),
                    amount: (20),
                }),
            },
            support::Extrinsic {
                caller: "alice".to_string().clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: (charlie),
                    amount: (20),
                }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                    claim: (&"HELLO!"),
                }),
            },
            support::Extrinsic {
                caller: "bob".to_string().clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
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
