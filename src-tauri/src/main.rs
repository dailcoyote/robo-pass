#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cryptography;
mod error;
mod kernel;
mod logger;

use std::fs;
use std::sync::Mutex;

use crate::logger::setup_logger;

use kernel::add_privacy;
use kernel::update_privacy;
use kernel::create_account;
use kernel::fetch_sorted_privacy_vec;
use kernel::remove_privacy;
use kernel::login;
use kernel::logout;
use kernel::can_user_access;
use kernel::UserSession;
use kernel::APP_FOLDER;

fn main() {
    if !APP_FOLDER.exists() {
        fs::create_dir(&*APP_FOLDER).expect("failed to create app folder");
    }

    let _ = setup_logger();

    tauri::Builder::default()
        .manage(Mutex::<Option<UserSession>>::default())
        .invoke_handler(tauri::generate_handler![
            create_account,
            add_privacy,
            update_privacy,
            fetch_sorted_privacy_vec,
            remove_privacy,
            login,
            logout,
            can_user_access
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
