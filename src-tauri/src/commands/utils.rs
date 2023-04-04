use local_ip_address::local_ip;
use tauri::Manager;
use walkdir::DirEntry;

use crate::SERVER_PORT;

//close the splash screen when this is called
#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// give connection details of the application core server
// get the ip address of the machine
#[tauri::command]
pub fn get_ip_address() -> String {
    format!(
        "{ip_address}:{port:?}",
        ip_address = local_ip().unwrap(),
        port = *SERVER_PORT
    )
}

// #[tauri::command]
// pub fn get_sys_information() -> Value {
//     json!({

//     })
// }
// a function to compute file size
// accept files size in byte and parse it to human readable KB, MB, TB, GB e.t.c
pub fn compute_file_size(size: u128) -> String {
    if size > (1024 * 1024 * 1024 * 1024) {
        format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024))
    } else if size > (1024 * 1024 * 1024) {
        format!("{:.2} GB", size / (1024 * 1024 * 1024))
    } else if size > (1024 * 1024) {
        format!("{:.2} MB", size / (1024 * 1024))
    } else if size > 1024 {
        format!("{:.2} KB", size / (1024))
    } else {
        format!("{:.2} B", size)
    }
}

// see if file is a dot file eg .cache .yarn
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
