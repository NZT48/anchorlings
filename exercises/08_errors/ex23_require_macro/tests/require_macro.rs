use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{instruction::Instruction, signature::Signer, transaction::Transaction};

use ex23_require_macro::{accounts, instruction, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex23_require_macro);

#[tokio::test]
async fn rejects_under_18() {
    let (mut banks, payer, blockhash) =
        ProgramTest::new("ex23_require_macro", PROGRAM_ID, processor!(entry))
            .start()
            .await;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::MinAge {}.to_account_metas(None),
        data: instruction::MinAge { age: 10 }.data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err(), "expected tx to fail for under-18 age");
}
