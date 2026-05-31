use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

use ex16_system_account_type::{accounts, instruction, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex16_system_account_type);

#[tokio::test]
async fn rejects_non_system_owned_account() {
    let bad_owner = Pubkey::new_unique();
    let weird_account = Pubkey::new_unique();

    let mut pt = ProgramTest::new("ex16_system_account_type", PROGRAM_ID, processor!(entry));
    pt.add_account(
        weird_account,
        Account {
            lamports: 1_000_000,
            data: vec![],
            owner: bad_owner,
            executable: false,
            rent_epoch: 0,
        },
    );

    let (mut banks, payer, blockhash) = pt.start().await;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::Check { system_owned: weird_account }.to_account_metas(None),
        data: instruction::Check {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let result = banks.process_transaction(tx).await;
    assert!(
        result.is_err(),
        "expected SystemAccount to reject an account whose owner isn't the System Program",
    );
}
