use anchor_lang::InstructionData;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

use ex17_program_type::{instruction, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex17_program_type);

#[tokio::test]
async fn rejects_wrong_program_pubkey() {
    let (mut banks, payer, blockhash) =
        ProgramTest::new("ex17_program_type", PROGRAM_ID, processor!(entry))
            .start()
            .await;

    // Some random pubkey pretending to be the System Program.
    let imposter = Pubkey::new_unique();

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(imposter, false)],
        data: instruction::InvokeSystem {}.data(),
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
        "expected Program<'info, System> to reject an account whose key isn't the System Program ID",
    );
}
