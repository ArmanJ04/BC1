// SPDX-License-Identifier: MIT
#![allow(dead_code)]

pub trait IERC165 {
    fn supports_interface(&self, interface_id: [u8; 4]) -> bool;
}

pub trait IERC721Receiver {
    fn on_erc721_received(&self, operator: &Address, from: &Address, token_id: u256, data: &[u8]) -> [u8; 4];
}

pub trait IERC721 {
    fn balance_of(&self, owner: &Address) -> u256;
    fn owner_of(&self, token_id: u256) -> Address;
}

pub trait IERC721Metadata {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn token_uri(&self, token_id: u256) -> String;
}

pub struct ERC165 {}

impl IERC165 for ERC165 {
    fn supports_interface(&self, interface_id: [u8; 4]) -> bool {
        interface_id == IERC165::interface_id()
    }
}

impl ERC165 {
    pub fn interface_id() -> [u8; 4] {
        [0, 0, 0, 0]
    }
}

pub struct MERC721 {
    token_id: u256,
    name: String,
    symbol: String,
    base_uri: String,
    owner: Address,
    balances: LookupMap<Address, u256>,
    owners: UnorderedMap<u256, Address>,
}

impl MERC721 {
    pub fn new(base_uri: String) -> Self {
        Self {
            token_id: 0,
            name: "TOPWEB3".to_string(),
            symbol: "TB3".to_string(),
            base_uri,
            owner: env::signer_account_id(),
            balances: LookupMap::new(b"balances".to_vec()),
            owners: UnorderedMap::new(b"owners".to_vec()),
        }
    }

    pub fn mint(&mut self, to: Address, provided_friends: u64) -> u256 {
        assert_eq!(env::signer_account_id(), self.owner, "ERC721: You are not the owner");
        assert!(provided_friends >= REQUIRED_FRIENDS, "ERC721: User does not have the required number of friends");

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

    pub fn token_uri(&self, token_id: u256) -> String {
        assert!(self.owners.contains_key(&token_id), "ERC721: URI query for nonexistent token");
        format!("{}{}", self.base_uri, token_id)
    }

    pub fn balance_of(&self, owner: Address) -> u256 {
        self.balances.get(&owner).unwrap_or(0)
    }

    pub fn owner_of(&self, token_id: u256) -> Address {
        self.owners.get(&token_id).unwrap_or_default()
    }
}
