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
use kernel::create_account;
use kernel::fetch_privacy_heap;
use kernel::UserSession;
use kernel::APP_FOLDER;

fn main() {
    if !APP_FOLDER.exists() {
        fs::create_dir(&*APP_FOLDER).expect("failed to create app folder");
    }

    // logs::initialize(&APP_FOLDER.join("logs")).expect("failed to initialize logger");
    // logs::remove_old(&APP_FOLDER.join("logs")).expect("failed to remove old logs");

    tauri::Builder::default()
        .manage(Mutex::<Option<UserSession>>::default())
        .invoke_handler(tauri::generate_handler![
            create_account,
            add_privacy,
            fetch_privacy_heap
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
