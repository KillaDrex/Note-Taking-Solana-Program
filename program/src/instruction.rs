use solana_program::program_error::ProgramError;
use borsh::BorshDeserialize;

pub enum NoteInstruction {
    AddNote {
        id: u16,
        title: String,
        body: String
    },
    UpdateNote {
        id: u16,
        title: String,
        body: String
    },
    DeleteNote {
        id: u16
    }
}

#[derive(BorshDeserialize)]
struct NotePayload {
    id: u16,
    title: String,
    body: String
}

impl NoteInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // split first byte and rest of bytes
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    
        // deserialize the rest of the data
        let payload = NotePayload::try_from_slice(rest).unwrap();
        
        // return success case with instruction instance embedded
        Ok(match variant {
            0 => Self::AddNote {
                id: payload.id,
                title: payload.title,
                body: payload.body
            },
            1 => Self::UpdateNote {
                id: payload.id,
                title: payload.title,
                body: payload.body
            },
            2 => Self::DeleteNote {
                id: payload.id
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}