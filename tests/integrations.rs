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
}