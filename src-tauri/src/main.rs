// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::os::unix::thread;
use chrono::{Datelike, Local};


use std::thread;




use sysinfo::{System, SystemExt};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};
mod channel_test;
mod arc_test;



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

fn get_week_of_year() -> String {
    let woy = Local::now().iso_week().week();
    return woy.to_string();
}

fn get_uptime() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut current_uptime = sys.uptime();
    current_uptime = current_uptime / 60;
    current_uptime = current_uptime / 1440;
    if current_uptime < 1 {
        current_uptime = 0
    }

    println!("current uptime! {}", current_uptime);
    current_uptime
}

fn add_a_thread() {
    thread::spawn(|| {
        println!("spawning thread");
    });
}

fn menu_conrad() -> SystemTraySubmenu {
    let uptime_menu_item = CustomMenuItem::new("uptime".to_string(), "N/A");
    let week_of_year = CustomMenuItem::new("woy".to_string(), "N/A");
    let add_thread_menu_item = CustomMenuItem::new("add_thread".to_string(), "Add A Thread");
    let menu_payload =  SystemTrayMenu::new()
    .add_item(week_of_year)
    .add_item(uptime_menu_item)
    .add_item(add_thread_menu_item);
    SystemTraySubmenu::new("Conrad",menu_payload)
}

fn main() {
    //TODO: This works but blocks the main thread...
    // channel_test::init();


    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(menu_conrad())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet, my_first_command])
        .on_system_tray_event(|app, event: SystemTrayEvent| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "add_thread" => {
                    println!("add thread clicked");
                    add_a_thread()
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick { tray_id, .. } => {
                println!("tray_id = {}", tray_id);
                let week_of_year = get_week_of_year();
                let mut week_of_year_payload = String::from("Week Of Year: ");
                week_of_year_payload.push_str(&week_of_year);

                let uptime = get_uptime();
                let mut uptime_payload = String::from("Uptime: ");
                uptime_payload.push_str(&uptime.to_string());
                app.tray_handle()
                    .get_item("woy")
                    .set_title(week_of_year_payload)
                    .unwrap();
                app.tray_handle()
                    .get_item("uptime")
                    .set_title(uptime_payload)
                    .unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
