fn main() {
    /// Scalars
    // Integers (i32)
    let day: u8 = 24; // 8 bits unsigned integer
    let answer: i32 = -42; // 32 bits signed integer
                           // Floats (f64)
    let gravity: f64 = 9.81; // 64 bits floating-point number
                             // Booleans
    let active: bool = true;
    // Characters
    let initial: char = 'R'; // Unicode Scalar Value

    /// Custom Types
    // Structs
    struct Point {
        x: f64,
        y: f64,
    }
    let origin = Point { x: 0.0, y: 0.0 };
    println!("Origin point coordinates: {}, {}", origin.x, origin.y);
    // Enums
    enum WebEvent {
        PageLoad,                 // A variant with no data
        KeyPress(String),         // A variant with associated data (String)
        Click { x: i32, y: i32 }, // A variant with associated data (Struct)
    }
    let response = WebEvent::KeyPress(String::from("Enter"));
    let response = WebEvent::Click { x: 10, y: -10 };
}
