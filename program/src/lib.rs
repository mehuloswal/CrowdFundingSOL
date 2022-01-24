use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

fn process_instruction(
    // program id is nothing but the id of this program on the solana network.
    program_id: &Pubkey,
    // When we invoke our program we can
    // give meta data of all the account we
    // want to work with.
    // As you can see it is a array of AccountInfo.
    // We can provide as many as we want.
    accounts: &[AccountInfo],
    // This is the data we want to process our instruction for.
    // It is a list of 8 bitunsigned integers(0..255).
    instruction_data: &[u8],
    // Here we specify the return type.
    // If you know a little bit of typescript.
    // This was of writing types and returns types might we familiar to you.
) -> ProgramResult {
    // And then since we can't return null in Rust we pass `Ok(())` to make it compile
    // It means the program executed successfully.

    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }
    msg!("Didnt find the entrypoint required");
    Err(ProgramError::InvalidInstructionData)

    // Ok(()) //Here the line Ok(()) is equivalent to return Ok(());
}

entrypoint!(process_instruction);

#[derive(Debug, BorshSerialize, BorshDeserialize)] //BorshSerialize is used to convert the struct into an array of u8, which is the data we can store in Solana accounts
struct CampaignDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub amount_donated: u64,
}

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    // Writing account or we can call it program account.
    // This is an account we will create in our front-end.
    // This account should br owned by the solana program.
    let writing_account = next_account_info(accounts_iter)?; //We can use ? Operator on the result enum to get the value
    let creator_account = next_account_info(accounts_iter)?;
    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }
    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by the program")
    }
}
fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
fn donate(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    Ok(())
}
