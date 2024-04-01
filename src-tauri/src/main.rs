#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cryptography;
mod error;
mod kernel;

use std::fs;
use std::sync::Mutex;

use kernel::add_privacy;
use kernel::update_privacy;
use kernel::create_account;
use kernel::fetch_privacy_heap;
use kernel::remove_privacy;
use kernel::login;
use kernel::logout;
use kernel::UserSession;
use kernel::APP_FOLDER;

fn main() {
    if !APP_FOLDER.exists() {
        fs::create_dir(&*APP_FOLDER).expect("failed to create app folder");
    }

    tauri::Builder::default()
        .manage(Mutex::<Option<UserSession>>::default())
        .invoke_handler(tauri::generate_handler![
            create_account,
            add_privacy,
            update_privacy,
            fetch_privacy_heap,
            remove_privacy,
            login,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
