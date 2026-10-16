
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ReviewPayload {
    pub title: String,
    pub location: String,
    pub rating: u8,
    pub description: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ReviewInstruction {
    Add(ReviewPayload),
    Update(ReviewPayload),
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }
}
