fn main() {
    /// INMUTABLE VARIABLE
    let sales_tax = 0.10;

    println!("Current tax rate is: {}", sales_tax);

    // If we try to change it, the compiler steps in:
    // sales_tax = 0.21; // <-- This line causes a compile-time ERROR

    
    /// MUTABLE VARIABLE
    let mut item_count = 10;
    
    println!("Initial count: {}", item_count);
    
    // We can reassign it because it was declared with 'mut'
    item_count = 15; // This is OK
    println!("New count: {}", item_count);


    /// SHADOWING
    // We try to use mutability to change the type
    // let mut x = "25"; 
    // x = x.parse().expect("Not a number"); // <-- This line causes a compile-time ERROR (cannot change type)

    // Shadowing is the correct way
    let raw_input = "25"; // Type: &str
    let raw_input = raw_input.parse::<u32>().expect("Not a number"); // Shadowing here! Type is now u32
    
    println!("The new type of 'raw_input' is: {}", raw_input);


    /// Borrowing
    let mut text = String::from("data");
    let r1 = &text; // First immutable borrow
    let r2 = &text; // Second immutable borrow

    // We can read through both
    println!("{}, {}", r1, r2);
    // We cannot modify 'text' or 'r1'/'r2' while these borrows are active.
    // text.push_str(" addition"); // <-- ERROR
    
    let mut message = String::from("hello");
    let m1 = &mut message; // First mutable borrow (OK)
    // let m2 = &mut message; // <-- COMPILE-TIME ERROR! (cannot borrow `message` as mutable more than once at a time)

    m1.push_str(" world"); // Modification is OK through m1
    println!("{}", m1);

    let mut data = String::from("original");
    let r3 = &data; // Immutable borrow (OK)
    let r4 = &data; // Another immutable borrow (OK)
    // let m3 = &mut data; // <-- COMPILE-TIME ERROR! (cannot borrow 'data' as mutable while r3 and r4 are active)
}
