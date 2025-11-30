use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // // Spawn creates a new thread
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // Main thread execution
    // for i in 1..5 {
    //     println!("Hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // // Waiting for the spawned thread to finish
    // handle.join().unwrap();
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // let (tx, rx) = mpsc::channel();

    // // Move allows that the thread takes ownership of 'tx'
    // thread::spawn(move || {
    //     let val = String::from("Hi from the thread!");
    //     // 'send' takes ownership of 'val'
    //     tx.send(val).unwrap(); 
    // });

    // // 'recv' blocks the main thread until data is received
    // let received = rx.recv().unwrap(); 

    // println!("Got: {}", received);
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 1. Arc allows multiple threads to safely own the data structure
    // 2. Mutex protects the internal data (integer) from concurrent access
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc for the new thread
        let handle = thread::spawn(move || {
            // Locking the Mutex is necessary to gain exclusive access
            let mut num = counter.lock().unwrap();
            *num += 1; // Mutate the protected data
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final result is guaranteed to be 10 due to Mutex ensuring exclusive access
    println!("Result: {}", *counter.lock().unwrap());
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
