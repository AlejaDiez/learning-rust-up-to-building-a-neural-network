// Function
fn add_one(number: i32) -> i32 {
    number + 1 // Without a semicolon, it’s considered an expression and is returned. It’s the same as ‘return number + 1;’.
}

// Implementation
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // A method that takes an immutable borrow (&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, scale: u32) {
        self.width *= scale;
        self.height *= scale;
    }

    // An associated function (static)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let result = add_one(10);

    println!("The result is: {}", result);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area is: {}", rect1.area());

    let sq = Rectangle::square(10);
    println!("Square size: {}", sq.width);
}
