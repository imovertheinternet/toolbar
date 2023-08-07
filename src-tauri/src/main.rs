// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use chrono::{Datelike, Local};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_first_command() -> String {
    println!("This message is inside the terminal");
    return format!("Returns a string");
}

fn main() {
    let week_of_year = Local::now().iso_week().week();
    let mut woy_payload = "Week Of Year: ".to_string();

    //TODO: will need to look into Arc and atomic integers. I think that allows me to have a constantly changing variable 
    // shared between threads.
    
    // let current_second_of_year = Local::now().timestamp_millis();
    // println!("second {}", current_second_of_year.to_string());

    //"&" Borrows from the inital WOY varaible. After flow control chapter itll talk about ownership/borrowing.
    woy_payload.push_str(&week_of_year.to_string());


    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("WOY".to_string(), woy_payload);
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet, my_first_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
