use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    native_token::LAMPORTS_PER_SOL,
   msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::create_account,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction_data {
    Initialize,
    Double,
    Halve,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let user_acc = next_account_info(&mut iter)?;
    let  data_account = next_account_info(&mut iter)?;
    let _system_program = next_account_info(&mut iter)?;
    let instruction_type = Instruction_data::try_from_slice(instruction_data)?;

    match instruction_type {
        Instruction_data::Initialize => {
            let ix = create_account(user_acc.key, data_account.key, LAMPORTS_PER_SOL, 8, program_id);
            invoke(&ix, accounts)?;
            let counter_data = Counter { count: 1 };
            let c = counter_data.serialize(&mut *data_account.data.borrow_mut())?;
        }
        Instruction_data::Double => {
            let mut counter_data = Counter::try_from_slice(&data_account.data.borrow())?;
         
            counter_data.count = counter_data.count * 2;
          let c=  counter_data.serialize(&mut *data_account.data.borrow_mut())?;

        }
        Instruction_data::Halve => {
            let mut counter_data = Counter::try_from_slice(&data_account.data.borrow())?;
            if data_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            counter_data.count = counter_data.count / 2;
            counter_data.serialize(&mut *data_account.data.borrow_mut())?;
        }
    };

    Ok(())
}
