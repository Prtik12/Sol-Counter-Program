use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut &mut acc.data.borrow_mut()[..])?;

    msg!("Contract succeeded");

    Ok(())
}
