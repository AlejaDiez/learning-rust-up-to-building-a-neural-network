enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

fn handle_message(msg: Message) {
    match msg {
        Message::Move { x, y } => {
            println!("Moved to x: {} and y: {}", x, y);
        }
        Message::Quit => {
            println!("Quit command received.");
        }
    }
}

fn main() {
    // IF
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // MATCH
    let coin = Coin::Dime;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => -1 // Matches any other value
    };

    println!("A dime is {} cents.", value);
    handle_message(Message::Move { x: 5, y: 10 });

    // LOOP
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // WHILE
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // FOR
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    for number in 1..4 {
        println!("{}!", number);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
