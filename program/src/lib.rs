// program id: HFhnUdFXZmqVzKr3C3BmS9ZJY4RARnWpHweAFh4S66EZ

pub mod instruction;
pub mod state;

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    system_instruction::create_account,
    borsh::try_from_slice_unchecked
};
use borsh::BorshSerialize;
use instruction::NoteInstruction;
use state::NoteAccountState;

// entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // deserialize instruction data
    let instruction = NoteInstruction::unpack(instruction_data)?;

    // determine what instruction to execute
    match instruction {
        NoteInstruction::AddNote{id, title, body} =>
            add_note(program_id, accounts, id, title, body),
        NoteInstruction::UpdateNote{id, title, body} =>
            update_note(program_id, accounts, id, title, body),
        NoteInstruction::DeleteNote{id} =>
            delete_note(program_id, accounts, id)
    }
}

fn add_note(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id: u16,
    title: String,
    body: String
) -> ProgramResult {
    // get accounts iterator
    let accounts_iter = &mut accounts.iter();

    // get accounts
    let initializer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // calculate pda and get bump seed
    let (pda, bump_seed) = 
        Pubkey::find_program_address(
            &[initializer.key.as_ref(), id.to_le_bytes().as_ref()],
            program_id
        );
    
    // calculate necessary size for pda account
    let account_size: usize = 1 + 2 + (4 + title.len() ) + (4 + body.len() );

    // calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_size);

    // create pda account
    invoke_signed(
        &create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_size.try_into().unwrap(),
            program_id
        ),
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(), id.to_le_bytes().as_ref(), &[bump_seed]]]
    )?;

    // log pda
    msg!("Derived PDA:{}", pda);

    // deserialize data field of newly created account
    let mut account_data = try_from_slice_unchecked::<NoteAccountState>(&pda_account.data.borrow()).unwrap();

    msg!("Borrowed account data");

    // update fields
    account_data.is_initialized = true;
    account_data.id = id;
    account_data.title = title;
    account_data.body = body;

    msg!("Updating account data");

    // serialize account
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    msg!("Initialized account");

    // log data
    msg!("Adding note...");
    msg!("ID:{}", account_data.id);
    msg!("Title:{}", account_data.title);
    msg!("Body:{}", account_data.body); 

    // return success
    Ok(())
}

fn update_note(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id: u16,
    title: String,
    body: String
) -> ProgramResult {
    // get accounts iterator
    let accounts_iter = &mut accounts.iter();

    // get accounts
    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // deserialize data field of newly created account
    let mut account_data = try_from_slice_unchecked::<NoteAccountState>(&pda_account.data.borrow()).unwrap();

    msg!("Borrowed account data");

    // update fields
    account_data.id = id;
    account_data.title = title;
    account_data.body = body;

    // serialize account
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    msg!("Updated fields");

    // log data
    msg!("Updated note...");
    msg!("ID:{}", account_data.id);
    msg!("Title:{}", account_data.title);
    msg!("Body:{}", account_data.body);

    // return success
    Ok(())
}

fn delete_note(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id: u16,
) -> ProgramResult {
    // get accounts iterator
    let accounts_iter = &mut accounts.iter();

    // get accounts
    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;    
   
    // get # of lamports in pda account
    let lamports_required = pda_account.lamports();
   
    // give the rent back to payer
    **pda_account.lamports.borrow_mut() -= lamports_required;
    **payer.lamports.borrow_mut() += lamports_required;

    // reallocate pda account storage to 0
    pda_account.realloc(0, true)?;

    // assign pda account to system program
    pda_account.assign(system_program.key);

    // log data
    msg!("Deleting note...");
    msg!("ID:{}", id);

    // return success
    Ok(())
}