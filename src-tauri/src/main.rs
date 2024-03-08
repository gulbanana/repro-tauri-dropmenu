// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    Manager, Window,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![menu])
        .menu(|app_handle| Menu::default(app_handle))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn menu(window: Window) {
    let manager = window.app_handle();
    let context_menu = Menu::with_items(
        manager,
        &[&MenuItem::with_id(manager, "item1", "Menu Item 1", true, None::<&str>).unwrap()],
    )
    .unwrap();

    window.popup_menu(&context_menu).unwrap();
}
