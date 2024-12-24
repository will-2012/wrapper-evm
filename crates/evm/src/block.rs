//! Block execution and state transition logic.
//!
//! Block execution is generally composed of the following steps:
//!
//! - System operations: Apply system transactions.
//! - Transaction execution: Execute transactions in the block.
//! - Post transaction execution: Apply post-transaction operations, (e.g. rewards, withdrawals).

// TODO: we need something similar to reth's executor trait that operates on a block, this crate
//  should provide the abstraction and reference implementation for optimism and ethereum, now that
//  we reuse all the alloy types in reth, we should be able to make use of all the alloy-consensus
//  types.

// TODO: ideally of this can also be reused for block building, although this could quickly become
//  complex unless we operate on something that yields `Tx`, and not blocks, this iterator style
//  type  must be aware of things like `gas_used`, invalid tx, etc... so that we can properly build
// a  block.
