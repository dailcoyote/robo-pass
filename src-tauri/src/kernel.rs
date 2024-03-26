// #[tauri::command]
// pub fn test(a: i8, b: i8) -> i8 {
//     a + b
// }

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::cryptography;
use crate::cryptography::EncryptedBlob;
use crate::error::Error;

pub static APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").expect("$APPDATA not set!");
        [&appdata, "robo-pass"].iter().collect()
    } else {
        let home = std::env::var("HOME").expect("$HOME not set!");
        [&home, ".config", "robo-pass"].iter().collect()
    }
});

#[derive(Default, Debug, Hash, Serialize, Deserialize, Clone)]
pub struct Privacy {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Keeper {
    pub unique_id: String,
    pub heap: HashMap<String, Privacy>,
}

impl Keeper {
    pub fn create(unique_id: String) -> Self {
        Self {
            unique_id,
            ..Self::default()
        }
    }
    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    pub fn add(&mut self, unique_id: String, url: String, username: String, password: String) {
        self.heap.insert(
            unique_id,
            Privacy {
                url,
                username,
                password,
            },
        );
    }

    pub fn remove(&mut self, unique_id: &str) -> bool {
        self.heap.remove(unique_id).is_some()
    }

    pub fn entry(&self, unique_id: &str) -> Option<&Privacy> {
        self.heap.get(unique_id)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &Privacy)> {
        self.heap.iter()
    }
}

#[derive(Default, Debug)]
pub struct UserSession {
    file: PathBuf,
    nonce: [u8; 16],
    encrypted_key: [u8; 32],
    key: [u8; 32],
    keeper: Keeper,
}

fn disk_dump(session: &UserSession) -> Result<(), Error> {
    println!("{:?}", session.keeper);
    let encrypted_blob = EncryptedBlob::encrypt(&session.keeper, &session.key)?;
    let file_content = session
        .nonce
        .iter()
        .copied()
        .chain(session.encrypted_key)
        .chain(encrypted_blob.bytes())
        .collect::<Vec<_>>();
    fs::write(&session.file, &file_content)?;
    Ok(())
}

#[tauri::command]
pub fn add_privacy(
    url: String,
    username: String,
    password: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<HashMap<String, Privacy>, Error> {
    if url.is_empty() || username.is_empty() || password.is_empty() {
        return Err(Error::InvalidReader);
    }
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    let unique_id = Uuid::new_v4().to_string();
    session.keeper.add(unique_id, url, username, password);
    disk_dump(session)?;
    Ok(session.keeper.heap.clone())
}

#[tauri::command]
pub fn fetch_privacy_heap(
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<HashMap<String, Privacy>, Error> {
    println!("Fetching credentials");
    let session_guard = session_mutex.lock()?;
    let session = session_guard.as_ref().ok_or(Error::InvalidReader)?;
    Ok(session.keeper.heap.clone())
}

#[tauri::command]
pub fn create_account(
    username: String,
    password: String,
    session: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    if username.is_empty() || password.is_empty() {
        return Err(Error::InvalidKeeper);
    }
    println!("Creating an account... {0}. {1},", username, password);
    let mut session = session.lock()?;
    if session.is_some() {
        return Err(Error::Unexpected);
    }
    let file = APP_FOLDER.join(format!("{username}.pwdb"));
    if file.exists() {
        return Err(Error::UsernameTaken);
    }
    let master_key = cryptography::pbkdf2_hmac(password.as_bytes(), username.as_bytes());
    let key = cryptography::random_bytes::<32>();
    let (encrypted_key, nonce) = cryptography::encrypt_key(&master_key, &key)?;
    let unique_id = Uuid::new_v4().to_string();
    let keeper = Keeper::create(unique_id);
    *session = Some(UserSession {
        file,
        nonce,
        encrypted_key,
        key,
        keeper,
    });
    disk_dump(session.as_ref().unwrap())?;
    println!("An account created");
    Ok(())
}
