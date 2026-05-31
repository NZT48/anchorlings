use anchor_lang::InstructionData;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

use ex13_signer_constraint::{instruction, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex13_signer_constraint);

#[tokio::test]
async fn rejects_when_owner_did_not_sign() {
    let (mut banks, payer, blockhash) =
        ProgramTest::new("ex13_signer_constraint", PROGRAM_ID, processor!(entry))
            .start()
            .await;

    // Pretend-owner — we'll deliberately *not* sign for this pubkey.
    let pretend_owner = Pubkey::new_unique();

    // Manually build account metas with `is_signer = false` so anchor's
    // `signer` constraint has something to actually reject.
    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(pretend_owner, false)],
        data: instruction::Withdraw {}.data(),
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
        "expected anchor to reject the call because `owner` did not sign",
    );
}
