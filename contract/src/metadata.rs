use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Funding {
    //owner of the funding
    pub owner_id: AccountId,
}