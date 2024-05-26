// SPDX-License-Identifier: MIT
use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Web3Linkedin {
    owner: String,
    post_counter: u64,
    users: LookupMap<String, UserProfile>,
    user_posts: LookupMap<String, Vector<Post>>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserProfile {
    name: String,
    bio: String,
    profile_picture_cid: String,
    friends: Vector<String>,
    friend_requests: Vector<String>,
    incoming_friend_requests: Vector<String>,
    has_minted_nft: bool,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Post {
    id: u64,
    author: String,
    content: String,
}

#[near_bindgen]
impl Web3Linkedin {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            post_counter: 0,
            users: LookupMap::new(b"u".to_vec()),
            user_posts: LookupMap::new(b"p".to_vec()),
        }
    }

    pub fn change_owner(&mut self, new_owner: String) {
        assert_eq!(env::signer_account_id(), self.owner, "Only contract owner can call this function");
        self.owner = new_owner;
    }

    pub fn set_nft(&mut self, user: String, value: bool) {
        let mut profile = self.get_or_default_profile(&user);
        assert!(!profile.has_minted_nft, "NFT status can only be set once");
        profile.has_minted_nft = value;
        self.users.insert(&user, &profile);
    }

    pub fn create_post(&mut self, content: String) {
        let author = env::signer_account_id();
        self.post_counter += 1;
        let new_post = Post {
            id: self.post_counter,
            author: author.clone(),
            content,
        };
        let mut user_posts = self.get_or_default_user_posts(&author);
        user_posts.push(&new_post);
        self.user_posts.insert(&author, &user_posts);
    }

    // Other contract functions...
    
    fn get_or_default_profile(&self, user: &String) -> UserProfile {
        match self.users.get(user) {
            Some(profile) => profile,
            None => UserProfile {
                name: "".to_string(),
                bio: "".to_string(),
                profile_picture_cid: "".to_string(),
                friends: Vector::new(b"f".to_vec()),
                friend_requests: Vector::new(b"fr".to_vec()),
                incoming_friend_requests: Vector::new(b"ifr".to_vec()),
                has_minted_nft: false,
            },
        }
    }

    fn get_or_default_user_posts(&self, user: &String) -> Vector<Post> {
        match self.user_posts.get(user) {
            Some(posts) => posts,
            None => Vector::new(b"p".to_vec()),
        }
    }
}
