use std::io;

fn main() {
    println!("Welcome to the Rust CLI Service!");
    loop {
        println!("Enter a string (or 'exit' to quit):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            break;
        }

        let transformed = transform_string(input);
        println!("Transformed result: {}", transformed);
    }
}

fn transform_string(input: &str) -> String {
    // Replace this with your custom transformation logic
    // For example, you can make it uppercase:
    input.to_uppercase()
}