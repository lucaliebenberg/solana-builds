use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct  UpdateArgs {
    pub value: u32
}

pub enum CounterInstructions {
    Increment,
    Decrement,
    Update(UpdateArgs),
    Reset,
}

