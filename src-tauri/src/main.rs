#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cryptography;
mod error;
mod kernel;
mod logger;
mod mem;

use std::fs;
use std::sync::Mutex;

use crate::logger::setup_logger;
use kernel::add_privacy;
use kernel::can_user_access;
use kernel::copy_to_clipboard;
use kernel::create_account;
use kernel::fetch_sorted_privacy_vec;
use kernel::generate_password;
use kernel::login;
use kernel::logout;
use kernel::remove_privacy;
use kernel::update_privacy;
use kernel::APP_FOLDER;
use log::info;
use mem::UserSession;

fn main() {
    if !APP_FOLDER.exists() {
        fs::create_dir(&*APP_FOLDER).expect("failed to create app folder");
    }

    let _ = setup_logger(&APP_FOLDER.join("logs")).expect("failed to initialize logger");

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
            can_user_access,
            generate_password,
            copy_to_clipboard
        ])
        .setup(|_app| {
            let unicode_backslash = "\u{5C}";
            let unicode_vertical = "\u{7C}";

            info!(" ______      _  ");
            info!(" | ___ {0}    | | ", unicode_backslash);
            info!(" | |_/ /___ | |__   ___    _ __   __ _ ___ ___  ");
            info!(
                " |    // _ {0}{1} '_ {0} / _ {0}  | '_ {0} / _` / __/ __| ",
                unicode_backslash, unicode_vertical
            );
            info!(
                " | |{0} {0} (_) | |_) | (_) | | |_) | (_| {0}__ {0}__ {0} ",
                unicode_backslash
            );
            info!(
                " {0}_{1} {0}_{0}___/{1}_.__/ {0}___/  {1} .__/ {0}__,_|___/___/ ",
                unicode_backslash, unicode_vertical
            );
            info!("                          | |                   ");
            info!("                          |_|                   ");

            info!("");
            info!("   _  _    __     __     __    ");
            info!("  ( )( )  |  |   /  {0}   /  {0}  ", unicode_backslash);
            info!("   {0}{0}//    )(  _( () )_( () )_ ", unicode_backslash);
            info!("   (__)   |__|(_){0}__/(_){0}__/(_)", unicode_backslash);
            info!("");

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Destroyed {} => {
              info!("Robo-pass application has been disconnected 🔌");
            }
            _ => {}
          })
        .run(tauri::generate_context!())
        .expect("error while running robo-pass application");
}
