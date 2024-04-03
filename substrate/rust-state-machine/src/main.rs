mod balances;
mod support;
mod system;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    use crate::{support, RuntimeCall};

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
    pub type Header = support::Block<BlockNumber, Extrinsic>;
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
