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
    pub keeper_id: String,
    pub privacy: Privacy,
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

    pub fn add(&mut self, uniqid: String, url: String, username: String, password: String) {
        self.heap.insert(
            uniqid,
            Privacy {
                url,
                username,
                password,
            },
        );
    }

    pub fn remove(&mut self, uniqid: &str) -> bool {
        self.heap.remove(uniqid).is_some()
    }

    pub fn update(&mut self, uniqid: &String, url: String, username: String, password: String) {
        match self.heap.get_mut(uniqid) {
            Some(privacy) => {
                *privacy = Privacy {
                    url,
                    username,
                    password,
                }
            } // If the key exists, update the value
            None => info!("Unique Id '{}' does not exist in the HashMap.", uniqid), // If the key does not exist, print a message
        }
    }

    pub fn entry(&self, uniqid: &str) -> Option<&Privacy> {
        self.heap.get(uniqid)
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
