use anchor_lang::{AccountDeserialize, AccountSerialize, InstructionData, ToAccountMetas};
use solana_program_test::*;
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

use ex19_multi_field_state::{accounts, instruction, Profile, ID as PROGRAM_ID};

anchorlings_test_kit::anchor_entry!(entry, ex19_multi_field_state);

fn make_profile_account(profile: Profile) -> Account {
    let mut data = Vec::new();
    profile.try_serialize(&mut data).unwrap();
    Account {
        lamports: 1_000_000,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

#[tokio::test]
async fn birthday_bumps_age_and_leaves_balance_alone() {
    let profile_key = Pubkey::new_unique();
    let mut pt = ProgramTest::new("ex19_multi_field_state", PROGRAM_ID, processor!(entry));
    pt.add_account(profile_key, make_profile_account(Profile { age: 30, balance: 100 }));

    let (mut banks, payer, blockhash) = pt.start().await;

    let ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: accounts::Birthday { profile: profile_key }.to_account_metas(None),
        data: instruction::Birthday {}.data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    banks.process_transaction(tx).await.unwrap();

    let account = banks.get_account(profile_key).await.unwrap().unwrap();
    let updated = Profile::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(updated.age, 31, "age should have been bumped");
    assert_eq!(updated.balance, 100, "balance should be untouched");
}
