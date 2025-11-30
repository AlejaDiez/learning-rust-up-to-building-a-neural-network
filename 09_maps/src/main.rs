use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Note the use of & to borrow the key

    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Team not found"),
    }
}
