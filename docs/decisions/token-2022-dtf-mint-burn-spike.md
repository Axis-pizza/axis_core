# Token-2022 DTF Mint/Burn Spike

## Purpose

Validate only the basic local Token-2022 mechanics Axis Core expects to depend on later: candidate DTF mint fixture creation, holder token-account fixture creation, authorized mint, authorized burn, supply reads, holder balance reads, and rejection of obvious authority/program-id misuse.

## Tested Fixture

- LiteSVM local runtime only.
- Test utility fixture in `axis-core-test-utils`.
- One candidate Token-2022 mint account.
- One holder Token-2022 token account for that mint.
- No Axis Core production instruction is invoked.
- No reserve vault, USDC source, USDC output, NAV, pricing, fee, route, DEX CPI, or public pool behavior is included.

## Tested Extension Set

The tested candidate is no extensions beyond base Token-2022 mint/account layout.

No transfer hooks, transfer fees, confidential transfer, permanent delegate, freeze authority, or other Token-2022 extensions are configured in this spike.

## Tested Authority Model

The tested fixture authority model is test-only:

- A funded deterministic test signer is the mint authority.
- A funded deterministic holder signer owns the holder token account.
- The holder signer authorizes burns from the holder token account.
- A separate funded deterministic signer is used for unauthorized mint and burn attempts.

Candidate Axis Core authority implications:

- Later Axis Core production work should validate that the DTF mint is owned by the Token-2022 program and controlled by the accepted Axis authority model.
- Later production minting should not use arbitrary externally held signing keys as final protocol authority.
- Later production burning should bind the burn authority to the expected user/account surface and redeem flow.

This fixture authority model is not final protocol design.

## Program IDs

- Token-2022: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
- Legacy SPL Token negative path: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- Axis Core production program: not invoked by this spike.

## Accounts Used

- Candidate DTF mint account owned by Token-2022.
- Candidate DTF holder token account owned by Token-2022.
- Test payer signer for account creation.
- Test mint authority signer.
- Test holder signer.
- Test unauthorized signer.

## Tests Added

- Create Token-2022 DTF mint and holder account fixture.
- Authorized mint increases mint supply and holder amount.
- Authorized burn decreases mint supply and holder amount.
- Unauthorized mint is rejected and does not change supply or holder amount.
- Unauthorized burn is rejected and does not change supply or holder amount.
- Legacy SPL Token program id is rejected for Token-2022-owned mint/account operations.
- Helper-level supply and holder amount mismatch assertions are covered.
- Base-layout mint/account data helpers reject uninitialized data.

## What This Proves

- This spike proves only basic Token-2022 mint/burn mechanics in LiteSVM.
- The local fixture can initialize a base Token-2022 mint and holder account.
- Token-2022 authorized mint changes mint supply and holder token-account amount as expected.
- Token-2022 authorized burn changes mint supply and holder token-account amount as expected.
- Obvious unauthorized mint and burn attempts are rejected by Token-2022.
- Using the legacy SPL Token program id against Token-2022-owned accounts is rejected in this harness.

## What This Does Not Prove

- This does not prove reserve-backed issuance.
- This does not prove Axis Core mint/redeem safety.
- This does not finalize production authority or extension policy.
- This does not prove NAV, target weights, pricing, fees, vault custody, route execution, ApprovedRoute, production DEX CPI, USDC input/output, or public DTF/USDC pool safety.
- This does not prove rollback behavior for future Axis Core instructions.
- This does not prove production mint/redeem account validation.

## Production Decisions Still Open

- Whether Axis Core creates DTF mints or validates externally initialized DTF mints for P0.
- Final production mint authority model and PDA signer derivation.
- Final production burn authority/account surface for redeem.
- Final Token-2022 extension allowlist or denylist for DTF mints.
- Whether freeze authority must be absent or controlled by a specific authority.
- How DTF mint validation binds to market state.
- How supply reads participate in reserve-backed mint/redeem accounting.
- Exact USDC, reserve vault, NAV, fee, route, and slippage validation rules.

## Follow-up Issues

- Implement production DTF mint authority validation.
- Define and enforce production Token-2022 extension policy.
- Wire DTF mint validation into market creation.
- Implement reserve-backed mint amount calculation from observed reserve deltas.
- Implement production redeem burn flow and reserve unwind validation.
- Add end-to-end LiteSVM tests for create_market, mint, redeem, and negative rollback cases once production instructions exist.
