use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // PANIC
    let index = 10;
    let items = vec![1, 2, 3];

    if index >= items.len() {
        // We use panic! for a bug that indicates a programmer error
        panic!("Index out of bounds! Index was {}, but length was {}", index, items.len());
    }

    // ERRORS
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    match greeting_file_result {
        // If successful, we get the File instance
        Ok(file) => {
            println!("File opened successfully!");
        }
        // If it failed, we get the Error struct
        Err(error) => {
            // Check the specific type of error
            if error.kind() == ErrorKind::NotFound {
                println!("File not found. Creating a new one...");
                // Attempt to create the file, handling that potential failure too
                File::create("hello.txt").unwrap(); 
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        }
    }
    // .unwrap(): If Result is Ok, return the value. If it is Err, call panic! macro
    // .expect("message"): It is the same as .unwrap(), but you can specify the message
}
