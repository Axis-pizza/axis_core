# Local Commands

This workspace pins Rust `1.93.1` in `rust-toolchain.toml` and requires the
`rustfmt` component.

## Format

```bash
cargo fmt --check
```

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

Run only the LiteSVM scaffold smoke tests:

```bash
cargo test -p axis-core-test-utils --test litesvm_smoke
```

## LiteSVM Program Artifact

The LiteSVM scaffold resolves the Axis Core program artifact explicitly:

```txt
AXIS_CORE_PROGRAM_ARTIFACT     optional override
target/deploy/axis_core.so     default workspace-relative path
```

If the artifact is missing, the smoke test reports the blocker and does not
pretend the program was loaded. If a file exists but is not a LiteSVM-loadable
Solana SBF shared object, the load helper returns an invalid-artifact
diagnostic.

## Solana / Pinocchio Build

The Pinocchio entrypoint feature can be checked with the host toolchain:

```bash
cargo build -p axis-core --features bpf-entrypoint
```

No Solana SBF build command is required or configured for this scaffold issue.
This command does not create `target/deploy/axis_core.so`. The program
entrypoint is gated behind the `bpf-entrypoint` feature so a later tooling issue
can add and verify the exact SBF build command.
