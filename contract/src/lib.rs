use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault,
};

mod web4;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    owner_id: AccountId,
    web4_ipfs_hash: Option<String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        web4_ipfs_hash: Option<String>
    ) -> Self {
        Self {
            owner_id,
            web4_ipfs_hash
        }
    }
}

impl Contract {
    pub fn assert_owner(&self) {
        assert_eq!(
            &self.owner_id,
            &env::predecessor_account_id(),
            "Not an owner!"
        );
    }
}