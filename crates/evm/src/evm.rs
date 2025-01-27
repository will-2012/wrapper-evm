//! Abstraction over EVM.

use alloy_primitives::{Address, Bytes};
use revm::{
    primitives::{BlockEnv, ResultAndState},
    Database, DatabaseCommit, GetInspector,
};

/// An instance of an ethereum virtual machine.
///
/// An EVM is commonly initialized with the corresponding block context and state and it's only
/// purpose is to execute transactions.
///
/// Executing a transaction will return the outcome of the transaction.
pub trait Evm {
    /// Database type held by the EVM.
    type DB: Database;
    /// The transaction object that the EVM will execute.
    type Tx;
    /// The error type that the EVM can return, in case the transaction execution failed, for
    /// example if the transaction was invalid.
    type Error;

    /// Reference to [`BlockEnv`].
    fn block(&self) -> &BlockEnv;

    /// Executes a transaction and returns the outcome.
    fn transact(&mut self, tx: Self::Tx) -> Result<ResultAndState, Self::Error>;

    /// Executes a system call.
    fn transact_system_call(
        &mut self,
        caller: Address,
        contract: Address,
        data: Bytes,
    ) -> Result<ResultAndState, Self::Error>;

    /// Returns a mutable reference to the underlying database.
    fn db_mut(&mut self) -> &mut Self::DB;

    /// Executes a transaction and commits the state changes to the underlying database.
    fn transact_commit(&mut self, tx_env: Self::Tx) -> Result<ResultAndState, Self::Error>
    where
        Self::DB: DatabaseCommit,
    {
        let result = self.transact(tx_env)?;
        self.db_mut().commit(result.state.clone());

        Ok(result)
    }
}

/// A type responsible for creating instances of an ethereum virtual machine given a certain input.
pub trait EvmFactory<Input> {
    /// The EVM type that this factory creates.
    // TODO: this doesn't quite work because this would force use to use an enum approach for trace
    // evm for example, unless we
    type Evm<'a, DB: Database + 'a, I: 'a>: Evm<DB = DB>;

    /// Creates a new instance of an EVM.
    fn create_evm<'a, DB: Database + 'a>(&self, db: DB, input: Input) -> Self::Evm<'a, DB, ()>;

    /// Creates a new instance of an EVM with an inspector.
    fn create_evm_with_inspector<'a, DB: Database + 'a, I: GetInspector<DB> + 'a>(
        &self,
        db: DB,
        input: Input,
        inspector: I,
    ) -> Self::Evm<'a, DB, I>;
}
