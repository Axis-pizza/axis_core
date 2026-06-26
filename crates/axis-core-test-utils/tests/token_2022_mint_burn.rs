use axis_core_test_utils::{
    assert_token_2022_amounts, burn_candidate_dtf_tokens, burn_candidate_dtf_tokens_with_program,
    create_candidate_token_2022_dtf_fixture, deterministic_keypair, fresh_vm,
    legacy_token_program_id, mint_candidate_dtf_tokens, mint_candidate_dtf_tokens_with_program,
    provision_funded_signer, token_2022_program_id, token_account_balance, token_mint_supply,
    AxisTestError, CANDIDATE_DTF_DECIMALS,
};
use solana_keypair::Signer;
use solana_program_pack::Pack;

#[test]
fn create_token_2022_dtf_mint_and_holder_account_fixture() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();

    assert_eq!(token_2022_program_id(), spl_token_2022_interface::id());
    assert_eq!(token_mint_supply(&vm, &fixture.mint()).unwrap(), 0);
    assert_eq!(
        token_account_balance(&vm, &fixture.holder_token_account()).unwrap(),
        0
    );
    assert_token_2022_amounts(&vm, &fixture.mint(), &fixture.holder_token_account(), 0, 0).unwrap();

    let mint_account = vm.get_account(&fixture.mint()).unwrap();
    let holder_account = vm.get_account(&fixture.holder_token_account()).unwrap();

    assert_eq!(mint_account.owner, token_2022_program_id());
    assert_eq!(holder_account.owner, token_2022_program_id());
    assert_eq!(
        mint_account.data.len(),
        spl_token_2022_interface::state::Mint::LEN
    );
    assert_eq!(
        holder_account.data.len(),
        spl_token_2022_interface::state::Account::LEN
    );
    assert_eq!(CANDIDATE_DTF_DECIMALS, 6);
}

#[test]
fn authorized_mint_increases_mint_supply_and_holder_amount() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();

    mint_candidate_dtf_tokens(
        &mut vm,
        &fixture,
        &fixture.mint_authority.keypair,
        1_250_000,
    )
    .unwrap();

    assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        1_250_000,
        1_250_000,
    )
    .unwrap();
}

#[test]
fn authorized_burn_decreases_mint_supply_and_holder_amount() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();

    mint_candidate_dtf_tokens(
        &mut vm,
        &fixture,
        &fixture.mint_authority.keypair,
        1_250_000,
    )
    .unwrap();
    burn_candidate_dtf_tokens(&mut vm, &fixture, &fixture.holder.keypair, 400_000).unwrap();

    assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        850_000,
        850_000,
    )
    .unwrap();
}

#[test]
fn unauthorized_mint_is_rejected_without_changing_supply_or_balance() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();
    let unauthorized =
        provision_funded_signer(&mut vm, "unauthorized-mint", 1_000_000_000).unwrap();

    mint_candidate_dtf_tokens(&mut vm, &fixture, &fixture.mint_authority.keypair, 100).unwrap();
    let err = mint_candidate_dtf_tokens(&mut vm, &fixture, &unauthorized.keypair, 50).unwrap_err();

    assert!(matches!(err, AxisTestError::TokenTransactionFailed { .. }));
    assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        100,
        100,
    )
    .unwrap();
}

#[test]
fn unauthorized_burn_is_rejected_without_changing_supply_or_balance() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();
    let unauthorized =
        provision_funded_signer(&mut vm, "unauthorized-burn", 1_000_000_000).unwrap();

    mint_candidate_dtf_tokens(&mut vm, &fixture, &fixture.mint_authority.keypair, 100).unwrap();
    let err = burn_candidate_dtf_tokens(&mut vm, &fixture, &unauthorized.keypair, 50).unwrap_err();

    assert!(matches!(err, AxisTestError::TokenTransactionFailed { .. }));
    assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        100,
        100,
    )
    .unwrap();
}

#[test]
fn legacy_token_program_is_rejected_for_token_2022_owned_mint_and_account() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();

    let mint_err = mint_candidate_dtf_tokens_with_program(
        &mut vm,
        &fixture,
        &fixture.mint_authority.keypair,
        100,
        &legacy_token_program_id(),
    )
    .unwrap_err();
    assert!(matches!(
        mint_err,
        AxisTestError::TokenTransactionFailed { .. }
    ));
    assert_token_2022_amounts(&vm, &fixture.mint(), &fixture.holder_token_account(), 0, 0).unwrap();

    mint_candidate_dtf_tokens(&mut vm, &fixture, &fixture.mint_authority.keypair, 100).unwrap();
    let burn_err = burn_candidate_dtf_tokens_with_program(
        &mut vm,
        &fixture,
        &fixture.holder.keypair,
        25,
        &legacy_token_program_id(),
    )
    .unwrap_err();

    assert!(matches!(
        burn_err,
        AxisTestError::TokenTransactionFailed { .. }
    ));
    assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        100,
        100,
    )
    .unwrap();
}

#[test]
fn helper_amount_assertion_covers_supply_and_holder_mismatch() {
    let mut vm = fresh_vm();
    let fixture = create_candidate_token_2022_dtf_fixture(&mut vm).unwrap();

    mint_candidate_dtf_tokens(&mut vm, &fixture, &fixture.mint_authority.keypair, 100).unwrap();

    let supply_err = assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        101,
        100,
    )
    .unwrap_err();
    assert_amount_mismatch(supply_err, "mint supply", 101, 100);

    let balance_err = assert_token_2022_amounts(
        &vm,
        &fixture.mint(),
        &fixture.holder_token_account(),
        100,
        101,
    )
    .unwrap_err();
    assert_amount_mismatch(balance_err, "holder token-account amount", 101, 100);
}

#[test]
fn mint_and_account_data_helpers_reject_uninitialized_base_layouts() {
    let mint_address = deterministic_keypair("uninitialized-token-2022-mint").pubkey();
    let account_address = deterministic_keypair("uninitialized-token-2022-account").pubkey();

    let mint_err =
        axis_core_test_utils::token_mint_supply_from_data(&mint_address, &[0_u8; 82]).unwrap_err();
    assert!(matches!(
        mint_err,
        AxisTestError::InvalidTokenMintData { .. }
    ));

    let account_err =
        axis_core_test_utils::token_account_balance_from_data(&account_address, &[0_u8; 165])
            .unwrap_err();
    assert!(matches!(
        account_err,
        AxisTestError::InvalidTokenAccountData { .. }
    ));
}

fn assert_amount_mismatch(err: AxisTestError, label: &str, expected: u64, actual: u64) {
    match err {
        AxisTestError::TokenAmountMismatch {
            label: actual_label,
            expected: actual_expected,
            actual: actual_actual,
        } => {
            assert_eq!(actual_label, label);
            assert_eq!(actual_expected, expected);
            assert_eq!(actual_actual, actual);
        }
        err => panic!("expected amount mismatch, got {err}"),
    }
}
