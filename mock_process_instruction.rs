// TL;DR
// 1. Simulates running a native (built-in) Solana instruction
// -> like transaction_accounts
// -> instruction_data
// -> builtin_function
// -> optional tweakes before execution like (pre_adjustments or post_adjustments)

// 2.  Internally, it:

// -> Calls mock_process_instruction_with_feature_set with all Solana features enabled (via SVMFeatureSet::all_enabled())
// -> Runs the instruction via the Solana runtime simulation (InvokeContext)
// -> Asserts the result matches expected_result
// -> Returns the final state of all involved account

pub fn mock_process_instruction<F: FnMut(&mut InvokeContext), G: FnMut(&mut InvokeContext)>(
    loader_id: &Pubkey,
    program_indices: Vec<IndexOfAccount>,
    instruction_data: &[u8],
    transaction_accounts: Vec<TransactionAccount>,
    instruction_account_metas: Vec<AccountMeta>,
    expected_result: Result<(), InstructionError>,
    builtin_function: BuiltinFunctionWithContext,
    pre_adjustments: F,
    post_adjustments: G,
) -> Vec<AccountSharedData> {
    mock_process_instruction_with_feature_set(
        loader_id,
        program_indices,
        instruction_data,
        transaction_accounts,
        instruction_account_metas,
        expected_result,
        builtin_function,
        pre_adjustments,
        post_adjustments,
        &SVMFeatureSet::all_enabled(),
    )
}