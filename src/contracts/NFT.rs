// SPDX-License-Identifier: MIT
use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MERC721 {
    token_id: u64,
    name: String,
    symbol: String,
    base_uri: String,
    owner: String,
    required_friends: u64,
    owners: UnorderedMap<u64, String>,
    balances: LookupMap<String, u64>,
}

#[near_bindgen]
impl MERC721 {
    #[init]
    pub fn new(base_uri: String) -> Self {
        Self {
            token_id: 0,
            name: "TOPWEB3".to_string(),
            symbol: "TB3".to_string(),
            base_uri,
            owner: env::signer_account_id(),
            required_friends: 2,
            owners: UnorderedMap::new(b"owners".to_vec()),
            balances: LookupMap::new(b"balances".to_vec()),
        }
    }

    pub fn mint(&mut self, to: String, provided_friends: u64) -> u64 {
        assert_eq!(env::signer_account_id(), self.owner, "ERC721: You are not the owner");
        assert!(provided_friends >= self.required_friends, "ERC721: User does not have the required number of friends");

        self.token_id += 1;
        self.owners.insert(&self.token_id, &to);
        self.balances.insert(&to, &self.balances.get(&to).unwrap_or(0) + 1);

        self.token_id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn get_token_uri(&self, token_id: u64) -> String {
        assert!(self.owners.contains_key(&token_id), "ERC721: URI query for nonexistent token");
        format!("{}{}", self.base_uri, token_id)
    }

    pub fn balance_of(&self, owner: String) -> u64 {
        self.balances.get(&owner).unwrap_or(0)
    }

    pub fn owner_of(&self, token_id: u64) -> String {
        self.owners.get(&token_id).unwrap_or("".to_string())
    }
}
