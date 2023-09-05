
use std::thread::sleep;
use std::time::Duration;
use std::{sync::Arc, thread};
use std::sync::Mutex;

pub fn arc_test() {
        // 1. Start with a string
    let woy_payload = String::from("Week of year:");

    //2. need to wrap it in an ARC to ensure safe concurrent access. Mutex will allow us exclusive access
    // to this string between threads to make sure no one steps on toes.
    let shared_woy_payload = Arc::new(Mutex::new(woy_payload));

    // 3. clone the arc instance so we can pass it to the thread. Effectively having multiple threads
    // share ownership. This clone references the same place in memory. THIS IS COPYING HEAP DATA, NOT JUST A REFERENCE.
    let thread_shared_woy_payload = shared_woy_payload.clone();

    // Create the thread and use move to MOVE ownership of the variables to inside the thread.
    let thread_handle = thread::spawn(move || {
        let when = Duration::from_secs(5);
        println!("inside of a thread");
        let mut counter = 0;
        loop {
            println!("before sleep");
            sleep(when);
            println!("after sleep");
            counter = counter + 1;
            println!("counter = {:?}", counter);
             let mut updated_woy_updated = thread_shared_woy_payload.lock().unwrap();
             updated_woy_updated.push_str("Add this to the end");
        }
    });

    /*
     * wait for thread to be done. then print final value
     * This is fine for a single sync call. What if i want multiple updates?
     */
    thread_handle.join().unwrap();
    println!("ouside of thread");
    println!("{}",shared_woy_payload.lock().unwrap());
}