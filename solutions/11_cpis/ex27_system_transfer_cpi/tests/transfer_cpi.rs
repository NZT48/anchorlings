use anchor_lang::{solana_program::system_program, InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
};

use ex27_system_transfer_cpi::{accounts, instruction, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex27_system_transfer_cpi);

#[tokio::test]
async fn forwards_lamports_to_receiver() {
    let (mut banks, payer, blockhash) =
        ProgramTest::new("ex27_system_transfer_cpi", PROGRAM_ID, processor!(entry))
            .start()
            .await;

    let receiver = Pubkey::new_unique();
    let amount: u64 = 1_000_000;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::Forward {
            from: payer.pubkey(),
            to: receiver,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::ForwardLamports { amount }.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    banks.process_transaction(tx).await.unwrap();

    let received = banks
        .get_account(receiver)
        .await
        .unwrap()
        .map(|a| a.lamports)
        .unwrap_or(0);
    assert_eq!(received, amount, "receiver should have been credited via the CPI");
}
