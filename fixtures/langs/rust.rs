// This is a Rust comment
struct Person {
    name: String,
    age: u32,
}

fn greet(name: &str) -> String {
    let message = format!("Hello, {}", name);
    message
}

fn main() {
    let user_name = "World";
    println!("{}", greet(user_name));
}
