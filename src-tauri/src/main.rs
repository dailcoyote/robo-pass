mod keeper;

use keeper::Keeper;
use std::collections::HashMap;

#[tauri::command]
fn submit(serializedKeepers: HashMap<String, Keeper>) {
    println!("{:?}", serializedKeepers);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![submit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
