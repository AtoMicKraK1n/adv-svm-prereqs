// TL;DR
// 1.Creates a mock runtime environment (InvokeContext) with dummy accounts.

// 2.Sets up instruction account mappings (InstructionAccounts).

// 3.Injects a native function (BuiltinFunctionWithContext).

// 4.Applies optional logic before and after execution (pre_adjustments & post_adjustments).

// 5.Calls invoke_context.process_instruction(...).

// 6.Asserts that the result equals the expected result.

// 7.Returns the final Vec<AccountSharedData> after execution.

#[allow(clippy::too_many_arguments)]
pub fn mock_process_instruction_with_feature_set<
    F: FnMut(&mut InvokeContext),
    G: FnMut(&mut InvokeContext),
>(
    loader_id: &Pubkey,
    mut program_indices: Vec<IndexOfAccount>,
    instruction_data: &[u8],
    mut transaction_accounts: Vec<TransactionAccount>,
    instruction_account_metas: Vec<AccountMeta>,
    expected_result: Result<(), InstructionError>,
    builtin_function: BuiltinFunctionWithContext,
    mut pre_adjustments: F,
    mut post_adjustments: G,
    feature_set: &SVMFeatureSet,
) -> Vec<AccountSharedData> {
    let mut instruction_accounts: Vec<InstructionAccount> =
        Vec::with_capacity(instruction_account_metas.len());
    for (instruction_account_index, account_meta) in instruction_account_metas.iter().enumerate() {
        let index_in_transaction = transaction_accounts
            .iter()
            .position(|(key, _account)| *key == account_meta.pubkey)
            .unwrap_or(transaction_accounts.len())
            as IndexOfAccount;
        let index_in_callee = instruction_accounts
            .get(0..instruction_account_index)
            .unwrap()
            .iter()
            .position(|instruction_account| {
                instruction_account.index_in_transaction == index_in_transaction
            })
            .unwrap_or(instruction_account_index) as IndexOfAccount;
        instruction_accounts.push(InstructionAccount {
            index_in_transaction,
            index_in_caller: index_in_transaction,
            index_in_callee,
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        });
    }
    if program_indices.is_empty() {
        program_indices.insert(0, transaction_accounts.len() as IndexOfAccount);
        let processor_account = AccountSharedData::new(0, 0, &native_loader::id());
        transaction_accounts.push((*loader_id, processor_account));
    }
    let pop_epoch_schedule_account = if !transaction_accounts
        .iter()
        .any(|(key, _)| *key == sysvar::epoch_schedule::id())
    {
        transaction_accounts.push((
            sysvar::epoch_schedule::id(),
            create_account_shared_data_for_test(&EpochSchedule::default()),
        ));
        true
    } else {
        false
    };
    with_mock_invoke_context_with_feature_set!(
        invoke_context,
        transaction_context,
        feature_set,
        transaction_accounts
    );
    let mut program_cache_for_tx_batch = ProgramCacheForTxBatch::default();
    program_cache_for_tx_batch.replenish(
        *loader_id,
        Arc::new(ProgramCacheEntry::new_builtin(0, 0, builtin_function)),
    );
    invoke_context.program_cache_for_tx_batch = &mut program_cache_for_tx_batch;
    pre_adjustments(&mut invoke_context);
    let result = invoke_context.process_instruction(
        instruction_data,
        &instruction_accounts,
        &program_indices,
        &mut 0,
        &mut ExecuteTimings::default(),
    );
    assert_eq!(result, expected_result);
    post_adjustments(&mut invoke_context);
    let mut transaction_accounts = transaction_context.deconstruct_without_keys().unwrap();
    if pop_epoch_schedule_account {
        transaction_accounts.pop();
    }
    transaction_accounts.pop();
    transaction_accounts
}
