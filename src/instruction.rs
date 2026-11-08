use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

use crate::{
    error::ReviewError,
    state::{MAX_DESCRIPTION_LEN, MAX_LOCATION_LEN, MAX_TITLE_LEN},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ReviewPayload {
    pub title: String,
    pub location: String,
    pub rating: u8,
    pub description: String,
}

impl ReviewPayload {
    /// Rating must be 1 to 5 and text fields must fit the account size limits (in bytes).
    pub fn validate(&self) -> Result<(), ProgramError> {
        if !(1..=5).contains(&self.rating) {
            return Err(ReviewError::InvalidRating.into());
        }
        if self.title.len() > MAX_TITLE_LEN {
            return Err(ReviewError::TitleTooLong.into());
        }
        if self.location.len() > MAX_LOCATION_LEN {
            return Err(ReviewError::LocationTooLong.into());
        }
        if self.description.len() > MAX_DESCRIPTION_LEN {
            return Err(ReviewError::DescriptionTooLong.into());
        }
        Ok(())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ReviewInstruction {
    Add(ReviewPayload),
    Update(ReviewPayload),
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(rating: u8) -> ReviewPayload {
        ReviewPayload {
            title: "Warung".into(),
            location: "Jakarta".into(),
            rating,
            description: "Great food".into(),
        }
    }

    #[test]
    fn roundtrips_instructions() {
        let bytes = ReviewInstruction::Add(payload(5)).try_to_vec().unwrap();
        assert!(matches!(
            ReviewInstruction::unpack(&bytes).unwrap(),
            ReviewInstruction::Add(p) if p.rating == 5
        ));
        assert!(ReviewInstruction::unpack(&[9, 9]).is_err());
    }

    #[test]
    fn validates_payload() {
        assert!(payload(1).validate().is_ok());
        assert!(payload(5).validate().is_ok());
        assert_eq!(payload(0).validate(), Err(ReviewError::InvalidRating.into()));
        assert_eq!(payload(6).validate(), Err(ReviewError::InvalidRating.into()));

        let mut long = payload(3);
        long.title = "x".repeat(MAX_TITLE_LEN + 1);
        assert_eq!(long.validate(), Err(ReviewError::TitleTooLong.into()));
    }
}
