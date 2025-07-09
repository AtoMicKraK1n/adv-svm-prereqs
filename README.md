# Understanding `invoke_context.rs` by Breaking it Down

To deeply understand the core Solana runtime logic around instruction execution, I decided to **break down** the massive and dense `invoke_context.rs` file into smaller, focused modules. This modular approach helps isolate different responsibilities and makes the code more navigable and testable.

---

## Breakdown of Files

### `invoke_context.rs`

- This is the **main logic file** that handles the execution of Solana instructions.
- It contains methods like `process_instruction`, stack management, compute metering, sysvar handling, and account access logic.
- It's the entry point when Solana executes any instruction — native or BPF.

---

### `struct_invoke_context.rs`

- Holds the **struct definition** of `InvokeContext` and all its fields.
- Useful to isolate state representation from behavior.
- Helps in understanding what runtime context is maintained while processing an instruction.

---

### `mock_process_instruction.rs`

- Provides a **test utility** that simulates running an instruction in a mocked environment.
- Used to test native instruction logic without needing a live validator or cluster.
- Accepts callbacks for pre/post adjustments to simulate edge cases.

---

### `mock_process_instruction_with_feature_set.rs`

- An extended version of `mock_process_instruction` that allows toggling Solana runtime **features** via `SVMFeatureSet`.
- Useful to test instruction behavior under different protocol upgrades or feature flags.

---

### `declare_process_instruction.rs`

- Contains a **macro** to simplify defining native Solana instruction handlers.
- Wraps the handler with compute unit metering and error handling.
- Used to register a Rust function as a native Solana program callable from the runtime.

---

## Why This Breakdown?

- The original `invoke_context.rs` mixes **state, behavior, test utilities, and macro logic** in one place.
- This modular approach gives better separation of concerns:
  - Struct vs. Logic
  - Runtime vs. Test code
  - Execution vs. Registration
