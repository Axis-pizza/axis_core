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

## Solana / Pinocchio Build

The Pinocchio entrypoint feature can be checked with the host toolchain:

```bash
cargo build -p axis-core --features bpf-entrypoint
```

No Solana SBF build command is required or configured for this scaffold issue.
The program entrypoint is gated behind the `bpf-entrypoint` feature so a later
tooling issue can add and verify the exact SBF build command.
