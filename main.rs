fn main() {
    let my_string = "hello";

    match my_string {
        "hello" => println!("Hello, world!"),
        "goodbye" => println!("Goodbye, world!"),
        _ => println!("Neither hello nor goodbye!"),
    }
}
