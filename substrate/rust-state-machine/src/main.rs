mod balances;
mod support;
mod system;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    use crate::{support, RuntimeCall};

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    /* TODO: Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`. */
    /* TODO: Define a concrete `Header` type using `BlockNumber`. */
    /* TODO: Define a concrete `Block` type using `Header` and `Extrinsic`. */
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    BalancesTransfer {
        to: types::AccountId,
        amount: types::Balance,
    },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
/* TODO: Add the derive macro to implement the `Debug` trait for `Runtime`. */
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
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

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
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

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
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
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(caller, to, amount)?;
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
        extrinsics: vec![support::Extrinsic {
            caller: "alice".to_string(),
            call: RuntimeCall::BalancesTransfer {
                to: "bob".to_string(),
                amount: 69,
            },
        }],
    };

    runtime.execute_block(block_1).expect("invalid block");

    println!("{:?}", runtime);
}
