use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Default, Debug, Hash, Serialize, Deserialize, Clone)]
pub struct Privacy {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PrivacySerialize {
    pub hash: String,
    pub credential: Privacy,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Keeper {
    pub username: String,
    pub heap: HashMap<String, Privacy>,
}

impl Keeper {
    pub fn create(username: String) -> Self {
        Self {
            username,
            ..Self::default()
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn add(&mut self, unique_hashtag: String, url: String, username: String, password: String) {
        self.heap.insert(
            unique_hashtag,
            Privacy {
                url,
                username,
                password,
            },
        );
    }

    pub fn remove(&mut self, unique_hashtag: &str) -> bool {
        self.heap.remove(unique_hashtag).is_some()
    }

    pub fn update(&mut self, unique_hashtag: &String, url: String, username: String, password: String) {
        match self.heap.get_mut(unique_hashtag) {
            Some(privacy) => {
                *privacy = Privacy {
                    url,
                    username,
                    password,
                }
            } // If the key exists, update the value
            None => info!("Unique Id '{}' does not exist in the HashMap.", unique_hashtag), // If the key does not exist, print a message
        }
    }

    pub fn entry(&self, unique_hashtag: &str) -> Option<&Privacy> {
        self.heap.get(unique_hashtag)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &Privacy)> {
        self.heap.iter()
    }
}

#[derive(Default, Debug)]
pub struct UserSession {
    pub file: PathBuf,
    pub nonce: [u8; 16],
    pub encrypted_key: [u8; 32],
    pub key: [u8; 32],
    pub keeper: Keeper,
}
