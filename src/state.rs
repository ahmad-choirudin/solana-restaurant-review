
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct AccountState {
    pub title: String,
    pub location: String,
    pub rating: u8,
    pub description: String,
}
