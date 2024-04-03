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
    // TODO: Not implemented yet.
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
        unimplemented!();
    }
}

fn main() {
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(&"alice".to_string(), 100);

    // start emulating a block
    /* TODO: Increment the block number in system. */
    runtime.system.inc_block_number();
    /* TODO: Assert the block number is what we expect. */
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    /* TODO: Increment the nonce of `alice`. */
    runtime.system.inc_nonce(&"alice".to_string());
    /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
        the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */
    let _res = runtime
        .balances
        .transfer("alice".to_string(), "bob".to_string(), 30)
        .map_err(|e| eprintln!("{}", e));

    // second transaction
    /* TODO: Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(&"alice".to_string());
    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
        .balances
        .transfer("alice".to_string(), "charlie".to_string(), 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:?}", runtime);
}
