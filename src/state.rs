use borsh::{BorshDeserialize, BorshSerialize};

pub const MAX_TITLE_LEN: usize = 100;
pub const MAX_LOCATION_LEN: usize = 100;
pub const MAX_DESCRIPTION_LEN: usize = 1000;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct AccountState {
    pub is_initialized: bool,
    pub initializer: [u8; 32],
    pub title: String,
    pub location: String,
    pub rating: u8,
    pub description: String,
}

impl AccountState {
    /// Allocate review accounts with at least this many bytes (worst case).
    pub const MAX_SIZE: usize = 1
        + 32
        + (4 + MAX_TITLE_LEN)
        + (4 + MAX_LOCATION_LEN)
        + 1
        + (4 + MAX_DESCRIPTION_LEN);
}
