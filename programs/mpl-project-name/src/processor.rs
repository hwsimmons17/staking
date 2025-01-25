use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::{
    handlers::{create::create, update::update},
    instruction::MplProjectNameInstruction,
};

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: MplProjectNameInstruction =
        MplProjectNameInstruction::try_from_slice(instruction_data)?;
    match instruction {
        MplProjectNameInstruction::Create(args) => {
            msg!("Instruction: Create");
            create(accounts, args)
        }
        MplProjectNameInstruction::Update(args) => {
            msg!("Instruction: Update");
            update(accounts, args)
        }
    }
}
