use rand::Rng;
use std::io::{self};

/* TODO: Add config feature */
fn main() {
    let mut buffer = String::new();

    println!("Enter string length");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    let length: u32 = buffer.trim().parse().expect("Enter a valid number!");

    println!(
        "Generated string({}): {}",
        length,
        generate_random_string(length)
    );
}

fn generate_random_string(length: u32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let char_as_bytes = chars.as_bytes();
    let mut random_string = String::new();

    for _ in 0..length {
        let random_number = rand::thread_rng().gen_range(0..chars.len());
        let char = char_as_bytes[random_number] as char;

        random_string.push(char)
    }

    random_string
}
