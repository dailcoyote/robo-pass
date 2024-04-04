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
            None => println!(
                "[robo-pass] Unique Id '{}' does not exist in the HashMap.",
                uniqid
            ), // If the key does not exist, print a message
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
    file: PathBuf,
    nonce: [u8; 16],
    encrypted_key: [u8; 32],
    key: [u8; 32],
    keeper: Keeper,
}

fn disk_dump(session: &UserSession) -> Result<(), Error> {
    println!("[robo-pass] Keeper dump {:?}", session.keeper);
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
) -> Result<String, Error> {
    if url.is_empty() || username.is_empty() || password.is_empty() {
        return Err(Error::InvalidReader);
    }
    println!("[robo-pass] Adding privacy");
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    let rand_uniqid = Uuid::new_v4().to_string();
    let privacy_id = rand_uniqid.clone();
    session.keeper.add(rand_uniqid, url, username, password);
    disk_dump(session)?;
    Ok(privacy_id)
}

#[tauri::command]
pub fn update_privacy(
    uniqid: String,
    url: String,
    username: String,
    password: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<bool, Error> {
    if uniqid.is_empty() || url.is_empty() || username.is_empty() || password.is_empty() {
        return Err(Error::InvalidReader);
    }
    println!("[robo-pass] Updating privacy by {0}", uniqid);
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    session.keeper.update(&uniqid, url, username, password);
    disk_dump(session)?;
    Ok(true)
}

#[tauri::command]
pub fn fetch_sorted_privacy_vec(
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<Vec<PrivacySerialize>, Error> {
    let session_guard = session_mutex.lock()?;
    let session = session_guard.as_ref().ok_or(Error::InvalidReader)?;
    println!(
        "[robo-pass] Fetching privacy data by keeper {:?}",
        session.keeper.username
    );
    let mut sorted_privacy_vec: Vec<PrivacySerialize> = session
        .keeper
        .entries()
        .map(|(keeper_id, privacy)| PrivacySerialize {
            keeper_id: keeper_id.to_string(),
            privacy: Privacy {
                url: privacy.url.to_string(),
                username: privacy.username.to_string(),
                password: privacy.password.to_string(),
            },
        })
        .collect();
    sorted_privacy_vec.sort_by(|a, b| a.privacy.url.cmp(&b.privacy.url));

    Ok(sorted_privacy_vec.clone())
}

#[tauri::command]
pub fn remove_privacy(
    uniqid: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    println!("[robo-pass] Removing privacy by {0}", uniqid);
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    if !session.keeper.remove(&uniqid) {
        return Err(Error::InvalidParameter);
    }
    disk_dump(session)?;
    Ok(())
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
    println!(
        "[robo-pass] Creating an account... {0}. {1},",
        username, password
    );
    let mut session = session.lock()?;
    if session.is_some() {
        println!("[robo-pass] Session exists");
        return Err(Error::Unexpected);
    }
    let file = APP_FOLDER.join(format!("{username}.pwdb"));
    if file.exists() {
        return Err(Error::UsernameTaken);
    }
    let master_key = cryptography::pbkdf2_hmac(password.as_bytes(), username.as_bytes());
    let key = cryptography::random_bytes::<32>();
    let (encrypted_key, nonce) = cryptography::encrypt_key(&master_key, &key)?;
    let keeper = Keeper::create(username);
    *session = Some(UserSession {
        file,
        nonce,
        encrypted_key,
        key,
        keeper,
    });
    disk_dump(session.as_ref().unwrap())?;
    println!("[robo-pass] An account created");
    Ok(())
}

#[tauri::command]
pub fn login(
    username: String,
    password: String,
    session: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    println!("[robo-pass] Logging in {0}", username);
    if username.is_empty() || password.is_empty() {
        return Err(Error::InvalidKeeper);
    }
    let mut session = session.lock()?;
    if session.is_some() {
        println!("[robo-pass] Session exists");
        return Err(Error::Unexpected);
    }
    let file = APP_FOLDER.join(format!("{username}.pwdb"));
    if !file.exists() {
        println!("[robo-pass] File doesn't exist");
        return Err(Error::InvalidKeeper);
    }
    let file_contents = fs::read(&file)?;
    if file_contents.len() < 16 + 32 + 1 {
        println!("[robo-pass] Broken bytes");
        return Err(Error::InvalidReader);
    }
    let nonce: [u8; 16] = file_contents[..16].try_into().unwrap();
    let encrypted_key: [u8; 32] = file_contents[16..16 + 32].try_into().unwrap();
    let master_key = cryptography::pbkdf2_hmac(password.as_bytes(), username.as_bytes());
    let key = cryptography::decrypt_key(&master_key, &encrypted_key, &nonce)
        .map_err(|_| Error::InvalidKeeper)?;
    let keeper: Keeper = EncryptedBlob::from_bytes(&file_contents[16 + 32..])?
        .decrypt(&key)
        .map_err(|_| Error::InvalidKeeper)?;
    if keeper.username() != username {
        println!("[robo-pass] User not found");
        return Err(Error::InvalidReader);
    }
    println!("[robo-pass] {:?}", keeper.clone());
    *session = Some(UserSession {
        file,
        nonce,
        encrypted_key,
        key,
        keeper,
    });
    println!("[robo-pass] {0} logged", username);
    Ok(())
}

#[tauri::command]
pub fn logout(session: State<'_, Mutex<Option<UserSession>>>) -> Result<(), Error> {
    let mut session = session.lock()?;
    println!("[robo-pass] Logging out {0}", logged_in = session.is_some());
    *session = None;
    Ok(())
}

#[tauri::command]
pub fn can_user_access(session: State<'_, Mutex<Option<UserSession>>>) -> Result<bool, Error> {
    let mut session = session.lock()?;
    Ok(session.is_some())
}
