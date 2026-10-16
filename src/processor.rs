
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    instruction::ReviewInstruction,
    state::AccountState,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = ReviewInstruction::unpack(input)?;
        let account_iter = &mut accounts.iter();
        let account = next_account_info(account_iter)?;

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !account.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }

        let state = match instruction {
            ReviewInstruction::Add(payload) => {
                if account.data_is_empty()
                    || account.try_borrow_data()?.iter().any(|b| *b != 0)
                {
                    return Err(ProgramError::AccountAlreadyInitialized);
                }

                AccountState {
                    title: payload.title,
                    location: payload.location,
                    rating: payload.rating,
                    description: payload.description,
                }
            }
            ReviewInstruction::Update(payload) => {
                {
                    let data = account.try_borrow_data()?;
                    let mut bytes: &[u8] = &data;
                    AccountState::deserialize(&mut bytes)
                        .map_err(|_| ProgramError::UninitializedAccount)?;
                }

                AccountState {
                    title: payload.title,
                    location: payload.location,
                    rating: payload.rating,
                    description: payload.description,
                }
            }
        };

        let serialized = state
            .try_to_vec()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let mut data = account.try_borrow_mut_data()?;

        if serialized.len() > data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        data.fill(0);
        data[..serialized.len()].copy_from_slice(&serialized);

        Ok(())
    }
              }
