use borsh::{BorshDeserialize, BorshSerialize};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use sol_counter_program::{InstructionType, Counter};

async fn test_increment_counter(){
    let program_id = Pubkey::new_unique();
    let counter_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "sol_counter_program",
        program_id,
        processor!(sol_counter_program::counter_contract),
    );

let counter_data = Counter { count: 0 };
let mut counter_account_data = vec![];
counter_data.serialize(&mut counter_account_data).unwrap();

program_test.add_account(
    counter_pubkey,
    Account {
        lamports: 1_000_000,
        data : counter_account_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    },
);

let (mut banks_client, payer, recent_blockhash) = program_test.start().await;



}