use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::ReviewError, instruction::ReviewInstruction, state::AccountState};

pub struct Processor;

impl Processor {
    /// Accounts: 0. `[signer]` review author, 1. `[writable]` review account owned by this program.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = ReviewInstruction::unpack(input)?;

        let account_iter = &mut accounts.iter();
        let initializer = next_account_info(account_iter)?;
        let account = next_account_info(account_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !account.is_writable {
            return Err(ProgramError::InvalidArgument);
        }

        // Existing account contents. A zero-filled account decodes as `is_initialized == false`.
        let existing = {
            let data = account.try_borrow_data()?;
            AccountState::deserialize(&mut &data[..])
                .map_err(|_| ProgramError::AccountDataTooSmall)?
        };

        let state = match instruction {
            ReviewInstruction::Add(payload) => {
                if existing.is_initialized {
                    return Err(ProgramError::AccountAlreadyInitialized);
                }
                payload.validate()?;
                AccountState {
                    is_initialized: true,
                    initializer: initializer.key.to_bytes(),
                    title: payload.title,
                    location: payload.location,
                    rating: payload.rating,
                    description: payload.description,
                }
            }
            ReviewInstruction::Update(payload) => {
                if !existing.is_initialized {
                    return Err(ProgramError::UninitializedAccount);
                }
                if existing.initializer != initializer.key.to_bytes() {
                    return Err(ReviewError::Unauthorized.into());
                }
                payload.validate()?;
                AccountState {
                    is_initialized: true,
                    initializer: existing.initializer,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::ReviewPayload;

    macro_rules! acct {
        ($name:ident, $key:expr, $signer:expr, $writable:expr, $owner:expr, $data:expr) => {
            let mut lamports = 0u64;
            let mut data = $data;
            let $name = AccountInfo::new(
                &$key, $signer, $writable, &mut lamports, &mut data, &$owner, false, 0,
            );
        };
    }

    fn payload(title: &str, rating: u8) -> ReviewPayload {
        ReviewPayload {
            title: title.into(),
            location: "Jakarta".into(),
            rating,
            description: "Great food".into(),
        }
    }

    fn read(account: &AccountInfo) -> AccountState {
        let data = account.data.borrow();
        AccountState::deserialize(&mut &data[..]).unwrap()
    }

    #[test]
    fn add_then_update_by_author() {
        let program_id = Pubkey::new_unique();
        let author = Pubkey::new_unique();
        let review_key = Pubkey::new_unique();
        let system = Pubkey::new_unique();
        acct!(init, author, true, false, system, Vec::<u8>::new());
        acct!(review, review_key, false, true, program_id, vec![0u8; AccountState::MAX_SIZE]);
        let accounts = [init, review.clone()];

        let add = ReviewInstruction::Add(payload("Warung", 4)).try_to_vec().unwrap();
        assert_eq!(Processor::process(&program_id, &accounts, &add), Ok(()));
        let s = read(&review);
        assert!(s.is_initialized);
        assert_eq!(s.initializer, author.to_bytes());
        assert_eq!(s.title, "Warung");

        // Adding again must fail.
        assert_eq!(
            Processor::process(&program_id, &accounts, &add),
            Err(ProgramError::AccountAlreadyInitialized)
        );

        let upd = ReviewInstruction::Update(payload("Warung Baru", 5)).try_to_vec().unwrap();
        assert_eq!(Processor::process(&program_id, &accounts, &upd), Ok(()));
        let s = read(&review);
        assert_eq!(s.title, "Warung Baru");
        assert_eq!(s.rating, 5);
    }

    #[test]
    fn update_by_someone_else_is_rejected() {
        let program_id = Pubkey::new_unique();
        let author = Pubkey::new_unique();
        let attacker = Pubkey::new_unique();
        let review_key = Pubkey::new_unique();
        let system = Pubkey::new_unique();
        acct!(init, author, true, false, system, Vec::<u8>::new());
        acct!(evil, attacker, true, false, system, Vec::<u8>::new());
        acct!(review, review_key, false, true, program_id, vec![0u8; AccountState::MAX_SIZE]);

        let add = ReviewInstruction::Add(payload("Warung", 4)).try_to_vec().unwrap();
        Processor::process(&program_id, &[init, review.clone()], &add).unwrap();

        let upd = ReviewInstruction::Update(payload("Hacked", 1)).try_to_vec().unwrap();
        assert_eq!(
            Processor::process(&program_id, &[evil, review.clone()], &upd),
            Err(ReviewError::Unauthorized.into())
        );
        assert_eq!(read(&review).title, "Warung");
    }

    #[test]
    fn rejects_missing_signature_bad_rating_and_uninitialized_update() {
        let program_id = Pubkey::new_unique();
        let author = Pubkey::new_unique();
        let review_key = Pubkey::new_unique();
        let system = Pubkey::new_unique();
        acct!(unsigned, author, false, false, system, Vec::<u8>::new());
        acct!(init, author, true, false, system, Vec::<u8>::new());
        acct!(review, review_key, false, true, program_id, vec![0u8; AccountState::MAX_SIZE]);

        let add = ReviewInstruction::Add(payload("Warung", 4)).try_to_vec().unwrap();
        assert_eq!(
            Processor::process(&program_id, &[unsigned, review.clone()], &add),
            Err(ProgramError::MissingRequiredSignature)
        );

        let bad = ReviewInstruction::Add(payload("Warung", 9)).try_to_vec().unwrap();
        assert_eq!(
            Processor::process(&program_id, &[init.clone(), review.clone()], &bad),
            Err(ReviewError::InvalidRating.into())
        );

        let upd = ReviewInstruction::Update(payload("Warung", 3)).try_to_vec().unwrap();
        assert_eq!(
            Processor::process(&program_id, &[init, review], &upd),
            Err(ProgramError::UninitializedAccount)
        );
    }

    #[test]
    fn empty_review_is_still_initialized() {
        // An all-empty payload used to serialize to all zero bytes and look "uninitialized".
        let program_id = Pubkey::new_unique();
        let author = Pubkey::new_unique();
        let review_key = Pubkey::new_unique();
        let system = Pubkey::new_unique();
        acct!(init, author, true, false, system, Vec::<u8>::new());
        acct!(review, review_key, false, true, program_id, vec![0u8; AccountState::MAX_SIZE]);

        let p = ReviewPayload {
            title: String::new(),
            location: String::new(),
            rating: 1,
            description: String::new(),
        };
        let add = ReviewInstruction::Add(p).try_to_vec().unwrap();
        assert_eq!(Processor::process(&program_id, &[init.clone(), review.clone()], &add), Ok(()));
        assert!(read(&review).is_initialized);
    }
}
