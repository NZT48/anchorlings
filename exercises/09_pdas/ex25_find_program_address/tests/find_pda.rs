use anchor_lang::{AccountSerialize, InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey,
    signature::{Keypair, Signer}, transaction::Transaction,
};

use ex25_find_program_address::{accounts, instruction, Vault, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex25_find_program_address);

fn make_vault_account(balance: u64) -> Account {
    let mut data = Vec::new();
    Vault { balance }.try_serialize(&mut data).unwrap();
    Account {
        lamports: 1_000_000,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

#[tokio::test]
async fn vault_is_per_user() {
    let user = Keypair::new();

    // The address the test client expects the program to derive: vault for
    // *this* user. If the program's seeds don't include the user, anchor
    // will derive some other address and reject the call.
    let (expected_vault, _bump) = Pubkey::find_program_address(
        &[b"vault", user.pubkey().as_ref()],
        &PROGRAM_ID,
    );

    let mut pt = ProgramTest::new("ex25_find_program_address", PROGRAM_ID, processor!(entry));
    pt.add_account(expected_vault, make_vault_account(100));

    let (mut banks, payer, blockhash) = pt.start().await;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::ReadVault {
            vault: expected_vault,
            user: user.pubkey(),
        }
        .to_account_metas(None),
        data: instruction::ReadVault {}.data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &user],
        blockhash,
    );
    banks.process_transaction(tx).await.expect(
        "expected anchor's derived PDA to match the user-scoped address we pre-baked",
    );
}
