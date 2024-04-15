use arboard::Clipboard;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::cryptography;
use crate::cryptography::EncryptedBlob;
use crate::error::Error;
use crate::mem::{Keeper, Privacy, PrivacySerialize, UserSession};

pub static APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").expect("$APPDATA not set!");
        [&appdata, "robo-pass"].iter().collect()
    } else {
        let home = std::env::var("HOME").expect("$HOME not set!");
        [&home, ".config", "robo-pass"].iter().collect()
    }
});

fn disk_dump(session: &UserSession) -> Result<(), Error> {
    info!(
        "Disk dump ✇ | {:?} user session saved to file: {:?} 💽",
        session.keeper.username, session.file
    );
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
    info!("Adding privacy for {0}", username);
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    let rand_hashtag = Uuid::new_v4().to_string();
    let unique_hashtag = rand_hashtag.clone();
    session.keeper.add(rand_hashtag, url, username, password);
    disk_dump(session)?;
    Ok(unique_hashtag)
}

#[tauri::command]
pub fn update_privacy(
    unique_hashtag: String,
    url: String,
    username: String,
    password: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<bool, Error> {
    if unique_hashtag.is_empty() || url.is_empty() || username.is_empty() || password.is_empty() {
        return Err(Error::InvalidReader);
    }
    info!("Updating privacy by {0}", unique_hashtag);
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    session
        .keeper
        .update(&unique_hashtag, url, username, password);
    disk_dump(session)?;
    Ok(true)
}

#[tauri::command]
pub fn fetch_sorted_privacy_vec(
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<Vec<PrivacySerialize>, Error> {
    let session_guard = session_mutex.lock()?;
    let session = session_guard.as_ref().ok_or(Error::InvalidReader)?;
    info!(
        "Fetching privacy data by keeper {:?}",
        session.keeper.username
    );
    let mut sorted_privacy_vec: Vec<PrivacySerialize> = session
        .keeper
        .entries()
        .map(|(unique_hashtag, privacy)| PrivacySerialize {
            hash: unique_hashtag.to_string(),
            credential: Privacy {
                url: privacy.url.to_string(),
                username: privacy.username.to_string(),
                password: privacy.password.to_string(),
            },
        })
        .collect();
    sorted_privacy_vec.sort_by(|a, b| a.credential.url.cmp(&b.credential.url));

    Ok(sorted_privacy_vec.clone())
}

#[tauri::command]
pub fn remove_privacy(
    unique_hashtag: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    info!("Removing privacy by {0}", unique_hashtag);
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    if !session.keeper.remove(&unique_hashtag) {
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
    info!("Creating an account for {0}", username);
    let mut session = session.lock()?;
    if session.is_some() {
        warn!("Session exists");
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
    warn!("An account created");
    Ok(())
}

#[tauri::command]
pub fn login(
    username: String,
    password: String,
    session: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    info!("Logging in {0}", username);
    if username.is_empty() || password.is_empty() {
        return Err(Error::InvalidKeeper);
    }
    let mut session = session.lock()?;
    if session.is_some() {
        warn!("Session exists");
        return Err(Error::Unexpected);
    }
    let file = APP_FOLDER.join(format!("{username}.pwdb"));
    if !file.exists() {
        error!("File doesn't exist");
        return Err(Error::InvalidKeeper);
    }
    let file_contents = fs::read(&file)?;
    if file_contents.len() < 16 + 32 + 1 {
        error!("Broken bytes");
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
        warn!("User not found");
        return Err(Error::InvalidReader);
    }
    debug!("Session |username| > {:?}", keeper.username());
    debug!("Session |encrypted_key| {:?}", encrypted_key);
    debug!("Session |key| {:?}", key);
    debug!("Session |file path| {:?}", file);
    *session = Some(UserSession {
        file,
        nonce,
        encrypted_key,
        key,
        keeper,
    });
    info!("Keeper |{0}| logged", username);
    Ok(())
}

#[tauri::command]
pub fn logout(session: State<'_, Mutex<Option<UserSession>>>) -> Result<(), Error> {
    let mut session = session.lock()?;
    info!("Logging out {0}", logged_in = session.is_some());
    *session = None;
    Ok(())
}

#[tauri::command]
pub fn can_user_access(session: State<'_, Mutex<Option<UserSession>>>) -> Result<bool, Error> {
    let session = session.lock()?;
    Ok(session.is_some())
}

#[tauri::command]
pub fn copy_to_clipboard(
    unique_hashtag: String,
    thing: String,
    session_mutex: State<'_, Mutex<Option<UserSession>>>,
) -> Result<(), Error> {
    debug!(
        "Copying {:?} to clipboard by hashtag {:?}",
        thing, unique_hashtag
    );
    let mut session_guard = session_mutex.lock()?;
    let session = session_guard.as_mut().ok_or(Error::InvalidReader)?;
    let entry = session.keeper.entry(&unique_hashtag).ok_or(Error::InvalidParameter)?;
    let text = match thing.as_str() {
        "username" => &entry.username,
        "password" => &entry.password,
        _ => return Err(Error::InvalidParameter),
    };
    Clipboard::new()?.set_text(text.clone())?;
    Ok(())
}

#[tauri::command]
pub fn generate_password() -> Result<String, Error> {
    debug!("Generating password");
    const PASSWORD_LENGTH: usize = 16;

    if !(10..=128).contains(&PASSWORD_LENGTH) {
        return Err(Error::InvalidParameter);
    }
    let alphabet: String = vec![
        String::from("abcdefghijklmnopqrstuvwxyz"),
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        String::from("0123456789"),
        String::from("!@#$%^&*"),
    ]
    .join("");

    Ok(cryptography::random_password(
        alphabet.as_bytes(),
        PASSWORD_LENGTH,
    ))
}
