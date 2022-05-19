use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{instruction::CounterInstruction, state::Counter};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::try_from_slice(&instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            CounterInstruction::Increment => {
                msg!("Incrementing counter");
                let mut accounts_iter = accounts.iter();
                let count_acc = next_account_info(&mut accounts_iter)?;
                let mut counter = Counter::try_from_slice(&count_acc.data.borrow())?;
                counter.value += 1;
                msg!("Updating count {}", counter.value);
                counter.serialize(&mut *count_acc.data.borrow_mut())?;
            }
            CounterInstruction::Decrement => {
                msg!("Decrementing counter");
                let mut accounts_iter = accounts.iter();
                let count_acc = next_account_info(&mut accounts_iter)?;
                let mut counter = Counter::try_from_slice(&count_acc.data.borrow())?;
                if counter.value != 0 {
                    counter.value -= 1;
                    msg!("Updating count {}", counter.value);
                }
                counter.serialize(&mut *count_acc.data.borrow_mut())?;
            }
        }
        Ok(())
    }
}
