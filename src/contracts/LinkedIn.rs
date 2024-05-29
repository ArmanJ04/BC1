// SPDX-License-Identifier: MIT
#![allow(dead_code)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Web3Linkedin {

    owner: Address,
    post_counter: u256,
    users: UnorderedMap<Address, UserProfile>,
    user_posts: UnorderedMap<Address, Vec<Post>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct UserProfile {
    name: String,
    bio: String,
    profile_picture_cid: String,
    friends: Vec<Address>,
    friend_requests: Vec<Address>,
    incoming_friend_requests: Vec<Address>,
    has_minted_nft: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Post {
    id: u256,
    author: Address,
    content: String,
}

impl Web3Linkedin {
    pub fn new() -> Self {
        Self {
            owner: env::signer_account_id(),
            post_counter: 0,
            users: UnorderedMap::new(b"users".to_vec()),
            user_posts: UnorderedMap::new(b"user_posts".to_vec()),
        }
    }

    pub fn get_friend_requests(&self, user: Address) -> Vec<Address> {
        self.users.get(&user).map(|profile| profile.friend_requests.clone()).unwrap_or_default()
    }

    pub fn get_incoming_friend_requests(&self, user: Address) -> Vec<Address> {
        self.users.get(&user).map(|profile| profile.incoming_friend_requests.clone()).unwrap_or_default()
    }

    pub fn get_user_profile(&self, user: Address) -> Option<UserProfile> {
        self.users.get(&user)
    }

    pub fn change_owner(&mut self, new_owner: Address) {
        self.owner = new_owner;
    }

    pub fn set_nft(&mut self, user: Address, value: bool) {
        if let Some(profile) = self.users.get_mut(&user) {
            profile.has_minted_nft = true;
        }
    }

    pub fn create_post(&mut self, content: String) {
        self.post_counter += 1;
        let new_post = Post {
            id: self.post_counter,
            author: msg.sender,
            content,
        };
        let user = self.users.entry(msg.sender).or_insert_with(Default::default);
        let user_posts = self.user_posts.entry(msg.sender).or_insert_with(Vec::new);
        user_posts.push(new_post.clone());
    }
    pub fn get_user_posts(&self, user: Address) -> Option<&Vec<Post>> {
        self.user_posts.get(&user)
    }

    pub fn register_profile(&mut self, user: Address, name: String) {
        if !self.users.contains_key(&user) {
            self.users.insert(&user, &UserProfile {
                name,
                bio: String::new(),
                profile_picture_cid: String::new(),
                friends: Vec::new(),
                friend_requests: Vec::new(),
                incoming_friend_requests: Vec::new(),
                has_minted_nft: false,
            });
        }
    }

    pub fn update_profile(&mut self, user: Address, name: String, bio: String, profile_picture_cid: String) {
        if let Some(profile) = self.users.get_mut(&user) {
            if !name.is_empty() && profile.name != name {
                profile.name = name;
            }
            if !bio.is_empty() && profile.bio != bio {
                profile.bio = bio;
            }
            if !profile_picture_cid.is_empty() && profile.profile_picture_cid != profile_picture_cid {
                profile.profile_picture_cid = profile_picture_cid;
            }
        }
    }

    pub fn send_friend_request(&mut self, from: Address, to: Address) {
        if from != to && !self.is_friend(&from, &to) && !self.is_friend_request_sent(&from, &to) {
            self.users.get_mut(&from).map(|profile| profile.friend_requests.push(to));
            self.users.get_mut(&to).map(|profile| profile.incoming_friend_requests.push(from));
        }
    }

    pub fn accept_friend_request(&mut self, from: Address, to: Address) {
        if self.is_friend_request_received(&from, &to) {
            self.users.get_mut(&to).map(|profile| profile.friends.push(from));
            self.users.get_mut(&from).map(|profile| profile.friends.push(to));
            self.remove_friend_request(&from, &to);
            self.remove_incoming_friend_request(&to, &from);
        }
    }

    pub fn decline_friend_request(&mut self, from: Address, to: Address) {
        if self.is_friend_request_received(&from, &to) {
            self.remove_friend_request(&from, &to);
            self.remove_incoming_friend_request(&to, &from);
        }
    }

    pub fn remove_friend(&mut self, user: Address, friend: Address) {
        if let Some(profile) = self.users.get_mut(&user) {
            if let Some(index) = profile.friends.iter().position(|&f| f == friend) {
                profile.friends.remove(index);
            }
        }
        if let Some(profile) = self.users.get_mut(&friend) {
            if let Some(index) = profile.friends.iter().position(|&f| f == user) {
                profile.friends.remove(index);
            }
        }
    }

    fn remove_friend_request(&mut self, user: Address, from: Address) {
        if let Some(profile) = self.users.get_mut(&user) {
            if let Some(index) = profile.friend_requests.iter().position(|&f| f == from) {
                profile.friend_requests.remove(index);
            }
        }
    }

    fn remove_incoming_friend_request(&mut self, user: Address, from: Address) {
        if let Some(profile) = self.users.get_mut(&user) {
            if let Some(index) = profile.incoming_friend_requests.iter().position(|&f| f == from) {
                profile.incoming_friend_requests.remove(index);
            }
        }
    }

    fn is_friend(&self, user1: &Address, user2: &Address) -> bool {
        self.users.get(&user1).map_or(false, |profile| profile.friends.contains(user2))
            && self.users.get(&user2).map_or(false, |profile| profile.friends.contains(user1))
    }

    fn is_friend_request_sent(&self, from: &Address, to: &Address) -> bool {
        self.users.get(&from).map_or(false, |profile| profile.friend_requests.contains(to))
    }

    fn is_friend_request_received(&self, from: &Address, to: &Address) -> bool {
        self.users.get(&to).map_or(false, |profile| profile.friend_requests.contains(from))
    }
}
