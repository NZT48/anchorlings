use anchor_lang::{AccountSerialize, AccountDeserialize, InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

use ex12_mut_constraint::{accounts, instruction, Counter, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex12_mut_constraint);

fn make_counter_account(value: u64) -> Account {
    let mut data = Vec::new();
    Counter { value }.try_serialize(&mut data).unwrap();
    Account {
        lamports: 1_000_000,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

#[tokio::test]
async fn set_value_persists() {
    let counter_key = Pubkey::new_unique();
    let mut pt = ProgramTest::new("ex12_mut_constraint", PROGRAM_ID, processor!(entry));
    pt.add_account(counter_key, make_counter_account(0));

    let (mut banks, payer, blockhash) = pt.start().await;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::SetValue { counter: counter_key }.to_account_metas(None),
        data: instruction::SetValue { value: 42 }.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    banks.process_transaction(tx).await.unwrap();

    let account = banks.get_account(counter_key).await.unwrap().unwrap();
    let counter = Counter::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(counter.value, 42);
}
