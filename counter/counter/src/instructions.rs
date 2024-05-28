use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

pub enum CounterInstructions {
    Increment,
    Decrement,
    Update,
    Reset
}