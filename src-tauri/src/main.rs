// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{Datelike, Local};
// use std::sync::{mpsc, Mutex};
// use std::thread::sleep;
// use std::time::Duration;
// use std::{sync::Arc, thread};
use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use async_std::channel;
use sysinfo::{System, SystemExt};

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

    println!("current uptime! {}", current_uptime);
    current_uptime
 
}

fn main() {
    // // 1. Start with a string
    // let woy_payload = String::from("Week of year:");

    // //2. need to wrap it in an ARC to ensure safe concurrent access. Mutex will allow us exclusive access
    // // to this string between threads to make sure no one steps on toes.
    // let shared_woy_payload = Arc::new(Mutex::new(woy_payload));

    // // 3. clone the arc instance so we can pass it to the thread. Effectively having multiple threads
    // // share ownership. This clone references the same place in memory. THIS IS COPYING HEAP DATA, NOT JUST A REFERENCE.
    // let thread_shared_woy_payload = shared_woy_payload.clone();

    // // Create the thread and use move to MOVE ownership of the variables to inside the thread.
    // let thread_handle = thread::spawn(move || {
    //     let when = Duration::from_secs(5);
    //     println!("inside of a thread");
    //     let mut counter = 0;
    //     loop {
    //         println!("before sleep");
    //         sleep(when);
    //         println!("after sleep");
    //         counter = counter + 1;
    //         println!("counter = {:?}", counter);
    //          let mut updated_woy_updated = thread_shared_woy_payload.lock().unwrap();
    //          updated_woy_updated.push_str("Add this to the end");
    //     }
    // });

    /*
     * wait for thread to be done. then print final value
     * This is fine for a single sync call. What if i want multiple updates?
     */
    // thread_handle.join().unwrap();
    // println!("ouside of thread");
    // println!("{}",shared_woy_payload.lock().unwrap());

    // // CHANNELS: Could do this instead of arc/mutex
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let when = Duration::from_secs(5);
    //     println!("inside of a thread");
    //     let mut counter = 0;
    //     loop {
    //         println!("before sleep");
    //         sleep(when);
    //         println!("after sleep");
    //         counter = counter + 1;
    //         tx.send(counter).unwrap();
    //         println!("counter = {:?}", counter);
    //     }
    // });

    /*
     * This handles multiple updates but blocks the main thread.
     */
    // for received_value in rx {
    //     println!("received: {}", received_value);
    //     let current_counter = received_value;
    // }

    // let mut received_value: Option<i32> = None;

    // loop {
    //     // Try to receive a value from the channel
    //     match rx.try_recv() {
    //         Ok(value) => {
    //             println!("found data! {}", value);
    //             // Update the received value
    //             received_value = Some(value);
    //         }
    //         Err(mpsc::TryRecvError::Empty) => {
    //             // No value available, do other non-blocking tasks here
    //             // println!("Main thread doing other work...");

    //             // Simulating non-blocking work
    //             // thread::sleep(Duration::from_millis(100));
    //         }
    //         Err(mpsc::TryRecvError::Disconnected) => {
    //             // Channel closed, break out of the loop
    //             break;
    //         }
    //     }
    // }

    // println!("after loop -=-=-=-=");
    // println!("received value {:?}", received_value);

    // // Now you can use the received value outside the loop
    // if let Some(value) = received_value {
    //     println!("Received outside the loop: {}", value);
    // } else {
    //     println!("No value received.");
    // }

    let uptime_menu_item = CustomMenuItem::new("uptime".to_string(), "n/a");
    let woy = CustomMenuItem::new("woy".to_string(), "n/a");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(woy)
        .add_item(uptime_menu_item)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet, my_first_command])
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                tray_id,
                ..
            } => {
                println!("tray_id = {}", tray_id);
                let week_of_year = get_week_of_year();
                let mut week_of_year_payload = String::from("Week Of Year = ");
                week_of_year_payload.push_str(&week_of_year);

                let uptime = get_uptime();
                let mut uptime_payload = String::from("Uptime =>");
                uptime_payload.push_str(&uptime.to_string());
                app.tray_handle().get_item("woy").set_title(week_of_year_payload).unwrap();
                app.tray_handle().get_item("uptime").set_title(uptime_payload).unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
