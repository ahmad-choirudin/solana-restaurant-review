use solana_program::program_error::ProgramError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReviewError {
    InvalidRating = 0,
    TitleTooLong = 1,
    LocationTooLong = 2,
    DescriptionTooLong = 3,
    Unauthorized = 4,
}

impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
