// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{Datelike, Local};
use std::sync::{mpsc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{sync::Arc, thread};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, Manager};

use async_std::channel;

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

fn main() {
    // let week_of_year = Local::now().iso_week().week();
    // let mut woy_payload = "Week Of Year: ".to_string();

    //TODO: will need to look into Arc and atomic integers. I think that allows me to have a constantly changing variable
    // shared between threads.

    // // 1. Start with a string
    // let woy_payload = String::from("Week of year:");

    // //2. need to wrap it in an ARC to ensure safe concurrent access. Mutex will allow us exclusive access
    // // to this string between threads to make sure no one steps on toes.
    // let shared_woy_payload = Arc::new(Mutex::new(woy_payload));

    // // 3. clone the arc instance so we can pass it to the thread. Effectively having multiple threads
    // // share ownership
    // let thread_shared_woy_payload = shared_woy_payload.clone();

    // // Create the thread and use move to MOVE ownership of the variables to inside the thread.
    // let thread_handle = thread::spawn(move || {
    //     let when = Duration::from_secs(5);
    //     println!("inside of a thread");
    //     loop {
    //         println!("inside loop");
    //         println!("before sleep");
    //         sleep(when);
    //         println!("after sleep");
    //          let mut updated_woy_updated = thread_shared_woy_payload.lock().unwrap();
    //          updated_woy_updated.push_str("Add this to the end");
    //     }

    //    // Now grab mutex and update the value
    //    let mut updated_woy_updated = thread_shared_woy_payload.lock().unwrap();
    //          updated_woy_updated.push_str("Add this to the end");
    // });

    // // wait for thread to be done. then print final value
    // thread_handle.join().unwrap();
    // println!("ouside of thread");
    // println!("{}",shared_woy_payload.lock().unwrap());

    // 1. Start with a string
    let woy_payload = String::from("Week of year:");

    //2. need to wrap it in an ARC to ensure safe concurrent access. Mutex will allow us exclusive access
    // to this string between threads to make sure no one steps on toes.
    let shared_woy_payload = Arc::new(Mutex::new(woy_payload));

    // 3. clone the arc instance so we can pass it to the thread. Effectively having multiple threads
    // share ownership
    let thread_shared_woy_payload = shared_woy_payload.clone();

    let (tx, rx) = mpsc::channel();
    // Create the thread and use move to MOVE ownership of the variables to inside the thread.
    let thread_handle = thread::spawn(move || {
        let when = Duration::from_secs(5);
        println!("inside of a thread");
        let mut counter = 0;

        loop {
            println!("inside loop");
            println!("before sleep");
            sleep(when);
            println!("after sleep");
            counter = counter + 1;
            tx.send(counter).unwrap();
            println!("counter = {:?}", counter);
            //  let mut updated_woy_updated = thread_shared_woy_payload.lock().unwrap();
            //  updated_woy_updated.push_str("Add this to the end");
        }
    });

    // wait for thread to be done. then print final value
    // thread_handle.join().unwrap();
    // println!("ouside of thread");
    // println!("{}",shared_woy_payload.lock().unwrap());

    // This works but blocks the main thread.
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

    println!("after loop -=-=-=-=");

    // // Now you can use the received value outside the loop
    // if let Some(value) = received_value {
    //     println!("Received outside the loop: {}", value);
    // } else {
    //     println!("No value received.");
    // }

    // let finalvalue = rx.recv().unwrap();
    // println!("final value {}", finalvalue.to_string());
    // thread::spawn(|| {
    //     // every 5 seconds hit this function forever.
    //     let wait_time = Duration::from_secs(5);
    //     let woy_payload = "Week Of Year:".to_string();
    //     loop {
    //         // Get current time

    //         // wait for 5 seconds
    //            println!("Before sleep");
    //         sleep(wait_time);
    //         println!("after sleep");
    //         // get week of year
    //         let mut woy = get_week_of_year();
    //         woy_payload.push_str(woy);
    //         println!("payload! {}", woy_payload);

    //         // // get the time
    //         // let mut current_time = Local::now();
    //         // let mut current_min_1 = Local::now().minute();
    //         // get_current_min();
    //         // println!("current time {:?}", current_time);
    //         // println!("min 1 {:?}", current_min_1);
    //         // // println!("min 2{:?}", current_min_2);
    //         // // has the minute increased?

    //         // // yes, call function to get new time

    //         // // no, do nothing.
    //         // println!("Before sleep");
    //         // sleep(wait_time);
    //         // println!("after sleep");
    //         // // Check if the date has changed.
    //     }
    // });

    // let current_second_of_year = Local::now().timestamp_millis();
    // println!("second {}", current_second_of_year.to_string());

    //"&" Borrows from the inital WOY varaible. After flow control chapter itll talk about ownership/borrowing.
    // woy_payload.push_str(&week_of_year.to_string());

    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    // let quit = CustomMenuItem::new("WOY".to_string(), woy_payload);
    let woy = CustomMenuItem::new("woy".to_string(), "n/a");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        // .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(woy)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet, my_first_command])
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { tray_id, position, size, .. } => {
                println!("waaaaaa");
                let updatedWOY = get_week_of_year();
                app.tray_handle().get_item("woy").set_title(updatedWOY);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
