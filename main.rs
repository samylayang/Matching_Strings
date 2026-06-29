fn main() {
    let my_string = "hello";

    match my_string {
        "hello" => println!("Hello, world!"),
        "goodbye" => println!("Goodbye, world!"),
        _ => println!("Neither hello nor goodbye!"),
    }

    let my_string: String = String::from("hello");

    match my_string.as_str() {
        "hello" => println!("Hello, world!"),
        "goodbye" => println!("Goodbye, world!"),
        _ => println!("Neither hello nor goodbye!"),
    }

    let my_string: String = String::from("HELLO");

    match my_string.to_lowercase().as_str() {
        "hello" => println!("Hello, world!"),
        "goodbye" => println!("Goodbye, world!"),
        _ => println!("Neither hello nor goodbye!"),
    }
}
