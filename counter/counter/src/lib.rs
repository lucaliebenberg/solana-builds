use borsh::{BorshDeserialize, BorshSerialize};
// use borsh::{BorshDeserialize, BorshSerialize};
// use borsh_derive::{BorshDeserialize, BorshSerialize};
use instructions::{
    CounterInstructions, 
    // UpdateArgs
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {

    msg!("Counter program entry point");

    let instructions: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instructions {
        CounterInstructions::Increment => {
            counter_account.counter += 1;
        }
        CounterInstructions::Decrement => {
            counter_account.counter -= 1;
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut() [..])?;
    Ok(())

}