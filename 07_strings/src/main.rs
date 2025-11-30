fn main() {
    let literal: &str = "Hello, world!";
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);
    println!("{}", literal);
}
