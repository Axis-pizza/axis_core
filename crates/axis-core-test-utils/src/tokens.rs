use litesvm::LiteSVM;
use solana_address::Address;
use solana_keypair::{Keypair, Signer};
use solana_program_pack::Pack;
use solana_transaction::Transaction;
use spl_token_2022_interface::{
    instruction as token_2022_instruction,
    state::{Account as TokenAccount, Mint},
};

use crate::{
    accounts::account_data, deterministic_keypair, provision_funded_signer, AxisTestError,
    FundedTestSigner, Result,
};

pub const CANDIDATE_DTF_DECIMALS: u8 = 6;

const TOKEN_MINT_MIN_LEN: usize = Mint::LEN;
const TOKEN_ACCOUNT_MIN_LEN: usize = 165;
const TOKEN_ACCOUNT_AMOUNT_OFFSET: usize = 64;
const TOKEN_ACCOUNT_AMOUNT_LEN: usize = 8;
const TOKEN_ACCOUNT_STATE_OFFSET: usize = 108;
const TOKEN_ACCOUNT_UNINITIALIZED_STATE: u8 = 0;
const TOKEN_FIXTURE_LAMPORTS: u64 = 1_000_000_000;

#[derive(Debug)]
pub struct CandidateToken2022DtfFixture {
    pub payer: FundedTestSigner,
    pub mint_authority: FundedTestSigner,
    pub holder: FundedTestSigner,
    pub mint_keypair: Keypair,
    pub holder_token_account_keypair: Keypair,
}

impl CandidateToken2022DtfFixture {
    pub fn mint(&self) -> Address {
        self.mint_keypair.pubkey()
    }

    pub fn holder_token_account(&self) -> Address {
        self.holder_token_account_keypair.pubkey()
    }
}

pub fn token_2022_program_id() -> Address {
    spl_token_2022_interface::id()
}

pub fn legacy_token_program_id() -> Address {
    spl_token_2022_interface::inline_spl_token::id()
}

pub fn create_candidate_token_2022_dtf_fixture(
    vm: &mut LiteSVM,
) -> Result<CandidateToken2022DtfFixture> {
    let payer = provision_funded_signer(vm, "token-2022-dtf-payer", TOKEN_FIXTURE_LAMPORTS)?;
    let mint_authority =
        provision_funded_signer(vm, "token-2022-dtf-mint-authority", TOKEN_FIXTURE_LAMPORTS)?;
    let holder = provision_funded_signer(vm, "token-2022-dtf-holder", TOKEN_FIXTURE_LAMPORTS)?;

    let mint_keypair = deterministic_keypair("token-2022-dtf-mint");
    let holder_token_account_keypair = deterministic_keypair("token-2022-dtf-holder-account");
    let token_program_id = token_2022_program_id();

    let create_mint_account = solana_system_interface::instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        vm.minimum_balance_for_rent_exemption(Mint::LEN),
        Mint::LEN as u64,
        &token_program_id,
    );
    let initialize_mint = token_2022_instruction::initialize_mint2(
        &token_program_id,
        &mint_keypair.pubkey(),
        &mint_authority.pubkey(),
        None,
        CANDIDATE_DTF_DECIMALS,
    )
    .map_err(instruction_build_failed)?;

    let create_holder_account = solana_system_interface::instruction::create_account(
        &payer.pubkey(),
        &holder_token_account_keypair.pubkey(),
        vm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
        TokenAccount::LEN as u64,
        &token_program_id,
    );
    let initialize_holder_account = token_2022_instruction::initialize_account3(
        &token_program_id,
        &holder_token_account_keypair.pubkey(),
        &mint_keypair.pubkey(),
        &holder.pubkey(),
    )
    .map_err(instruction_build_failed)?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint_account,
            initialize_mint,
            create_holder_account,
            initialize_holder_account,
        ],
        Some(&payer.pubkey()),
        &[&payer.keypair, &mint_keypair, &holder_token_account_keypair],
        vm.latest_blockhash(),
    );

    send_token_transaction(vm, transaction)?;

    Ok(CandidateToken2022DtfFixture {
        payer,
        mint_authority,
        holder,
        mint_keypair,
        holder_token_account_keypair,
    })
}

pub fn mint_candidate_dtf_tokens(
    vm: &mut LiteSVM,
    fixture: &CandidateToken2022DtfFixture,
    authority: &Keypair,
    amount: u64,
) -> Result<()> {
    mint_candidate_dtf_tokens_with_program(vm, fixture, authority, amount, &token_2022_program_id())
}

pub fn mint_candidate_dtf_tokens_with_program(
    vm: &mut LiteSVM,
    fixture: &CandidateToken2022DtfFixture,
    authority: &Keypair,
    amount: u64,
    token_program_id: &Address,
) -> Result<()> {
    let mint_to = token_2022_instruction::mint_to(
        token_program_id,
        &fixture.mint(),
        &fixture.holder_token_account(),
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(instruction_build_failed)?;

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to],
        Some(&authority.pubkey()),
        &[authority],
        vm.latest_blockhash(),
    );

    send_token_transaction(vm, transaction)
}

pub fn burn_candidate_dtf_tokens(
    vm: &mut LiteSVM,
    fixture: &CandidateToken2022DtfFixture,
    authority: &Keypair,
    amount: u64,
) -> Result<()> {
    burn_candidate_dtf_tokens_with_program(vm, fixture, authority, amount, &token_2022_program_id())
}

pub fn burn_candidate_dtf_tokens_with_program(
    vm: &mut LiteSVM,
    fixture: &CandidateToken2022DtfFixture,
    authority: &Keypair,
    amount: u64,
    token_program_id: &Address,
) -> Result<()> {
    let burn = token_2022_instruction::burn(
        token_program_id,
        &fixture.holder_token_account(),
        &fixture.mint(),
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(instruction_build_failed)?;

    let transaction = Transaction::new_signed_with_payer(
        &[burn],
        Some(&authority.pubkey()),
        &[authority],
        vm.latest_blockhash(),
    );

    send_token_transaction(vm, transaction)
}

pub fn assert_token_2022_amounts(
    vm: &LiteSVM,
    mint: &Address,
    token_account: &Address,
    expected_supply: u64,
    expected_account_amount: u64,
) -> Result<()> {
    assert_token_amount("mint supply", expected_supply, token_mint_supply(vm, mint)?)?;
    assert_token_amount(
        "holder token-account amount",
        expected_account_amount,
        token_account_balance(vm, token_account)?,
    )
}

pub fn token_mint_supply(vm: &LiteSVM, mint: &Address) -> Result<u64> {
    let data = account_data(vm, mint)?;
    token_mint_supply_from_data(mint, &data)
}

pub fn token_mint_supply_from_data(address: &Address, data: &[u8]) -> Result<u64> {
    if data.len() < TOKEN_MINT_MIN_LEN {
        return Err(AxisTestError::InvalidTokenMintData {
            address: *address,
            reason: format!(
                "expected at least {TOKEN_MINT_MIN_LEN} bytes, got {} bytes",
                data.len()
            ),
        });
    }

    let mint = Mint::unpack(&data[..TOKEN_MINT_MIN_LEN]).map_err(|err| {
        AxisTestError::InvalidTokenMintData {
            address: *address,
            reason: format!("failed to unpack base mint layout: {err:?}"),
        }
    })?;

    if !mint.is_initialized {
        return Err(AxisTestError::InvalidTokenMintData {
            address: *address,
            reason: "mint state is uninitialized".to_string(),
        });
    }

    Ok(mint.supply)
}

/// Token-2022 extensions append data after the canonical 165-byte token account
/// base layout, so this helper only inspects the shared amount field.
pub fn token_account_balance(vm: &LiteSVM, token_account: &Address) -> Result<u64> {
    let data = account_data(vm, token_account)?;
    token_account_balance_from_data(token_account, &data)
}

pub fn token_account_balance_from_data(address: &Address, data: &[u8]) -> Result<u64> {
    if data.len() < TOKEN_ACCOUNT_MIN_LEN {
        return Err(AxisTestError::InvalidTokenAccountData {
            address: address.clone(),
            reason: format!(
                "expected at least {TOKEN_ACCOUNT_MIN_LEN} bytes, got {} bytes",
                data.len()
            ),
        });
    }

    if data[TOKEN_ACCOUNT_STATE_OFFSET] == TOKEN_ACCOUNT_UNINITIALIZED_STATE {
        return Err(AxisTestError::InvalidTokenAccountData {
            address: address.clone(),
            reason: "token account state is uninitialized".to_string(),
        });
    }

    let amount_end = TOKEN_ACCOUNT_AMOUNT_OFFSET + TOKEN_ACCOUNT_AMOUNT_LEN;
    let amount = data[TOKEN_ACCOUNT_AMOUNT_OFFSET..amount_end]
        .try_into()
        .expect("token amount slice length is fixed");

    Ok(u64::from_le_bytes(amount))
}

fn assert_token_amount(label: &str, expected: u64, actual: u64) -> Result<()> {
    if actual != expected {
        return Err(AxisTestError::TokenAmountMismatch {
            label: label.to_string(),
            expected,
            actual,
        });
    }

    Ok(())
}

fn send_token_transaction(vm: &mut LiteSVM, transaction: Transaction) -> Result<()> {
    vm.send_transaction(transaction).map(|_| ()).map_err(|err| {
        AxisTestError::TokenTransactionFailed {
            details: format!("{err:?}"),
        }
    })
}

fn instruction_build_failed(err: impl std::fmt::Debug) -> AxisTestError {
    AxisTestError::TokenInstructionBuildFailed {
        details: format!("{err:?}"),
    }
}
