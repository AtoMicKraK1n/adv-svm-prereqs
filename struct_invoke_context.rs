// this is merely an overview how transactions are executing,
// will need to go through it to understand how actually things going through pipeline

pub struct InvokeContext<'a> {
    /// Information about the currently executing transaction.
    pub transaction_context: &'a mut TransactionContext,
    /// The local program cache for the transaction batch.
    pub program_cache_for_tx_batch: &'a mut ProgramCacheForTxBatch,
    /// Runtime configurations used to provision the invocation environment.
    pub environment_config: EnvironmentConfig<'a>,
    /// The compute budget for the current invocation.
    compute_budget: SVMTransactionExecutionBudget,
    /// The compute cost for the current invocation.
    execution_cost: SVMTransactionExecutionCost,
    /// Instruction compute meter, for tracking compute units consumed against
    /// the designated compute budget during program execution.
    compute_meter: RefCell<u64>,
    log_collector: Option<Rc<RefCell<LogCollector>>>,
    /// Latest measurement not yet accumulated in [ExecuteDetailsTimings::execute_us]
    pub execute_time: Option<Measure>,
    pub timings: ExecuteDetailsTimings,
    pub syscall_context: Vec<Option<SyscallContext>>,
    traces: Vec<Vec<[u64; 12]>>,
}

// TransactionContext
// 1. this mainly depends on InstructionAccount struct (things like index_in_transaction, index_in_caller, index_in_callee)
// 2. has two main functions (update_accounts_resize_delta, can_data_be_resized)
//    first one helps for updating the accounts by adding new_len and subtracting old_len
//    second one helps for checking the accounts like are they resizeable or not

