fn main() {
    // Dynamic arrays
    let mut numbers = vec![1, 2, 3];

    numbers.push(4);
    numbers.push(5);

    // println!("{}", number[7]); // Unsafe handling, panic if it doesn't exist
    match numbers.get(7) {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no eighth value"), // Safe handling
    }

    println!("Numbers: {:?}", numbers);
}
