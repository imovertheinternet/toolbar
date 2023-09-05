
use std::{
    thread::{self, sleep},
    time::Duration,
};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

fn start_recv_thread(rx: &Receiver<i32>) -> i32 {
    rx.recv().unwrap()

    // let mut received_value: Option<i32> = None;

    // let t = thread::spawn(move || {
    //     println!("starting recevier thread!");
    //     match rx.try_recv() {
    //         Ok(value) => {
    //             println!("found data! {:?}", value);
    //             // Update the received value
    //             // received_value = Some(value);
    //         }
    //         Err(errz) => {
    //             println!("error from recv {:?}", errz)
    //             // No value available, do other non-blocking tasks here
    //             // println!("Main thread doing other work...");

    //             // Simulating non-blocking work
    //             // thread::sleep(Duration::from_millis(100));
    //         }
    //         Err(mpsc::TryRecvError::Disconnected) => {
    //             // Channel closed, break out of the loop
    //             // break;
    //         }
    //     }
    // });

    // t.join().unwrap();

    // loop {
    //     // Try to receive a value from the channel
    //     match rx.try_recv() {
    //         // Ok(value) => {
    //         //     println!("found data! {}", value);
    //         //     // Update the received value
    //         //     received_value = Some(value);
    //         // }
    //         // Err(mpsc::TryRecvError::Empty) => {
    //         //     // No value available, do other non-blocking tasks here
    //         //     // println!("Main thread doing other work...");

    //         //     // Simulating non-blocking work
    //         //     // thread::sleep(Duration::from_millis(100));
    //         // }
    //         // Err(mpsc::TryRecvError::Disconnected) => {
    //         //     // Channel closed, break out of the loop
    //         //     break;
    //         // }
    //     }
    // }
}



pub fn init(){
    // CHANNELS: Could do this instead of arc/mutex
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let when = Duration::from_secs(3);
		println!("inside of a thread");
		let mut counter = 0;
		loop {
			println!("before sleep");
			sleep(when);
			println!("after sleep");
			counter = counter + 1;
			println!("counter = {:?}", counter);
			// tx.send(counter).unwrap();
			match tx.send(counter) {
				Ok(_) => {
					println!("ok");
				}
				Err(err) => println!("error!! {:?}", err)
			}

		}
	});

	loop {
		println!("inside loop");
	    let received_message = start_recv_thread(&rx);
	    println!("REC MESSAGE! {:?}", received_message);
	}
}
