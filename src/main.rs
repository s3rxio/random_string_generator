use clap::Parser;
use rand::Rng;
use std::format as fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version="0.1.0", about="random_string_generator", long_about = None)]
struct Options {
    #[arg(short = 'l', long = "length", help = "Length of the generated string")]
    length: u16,
}

fn main() {
    let args = Options::parse();

    let random_string = generate_random_string(args.length);

    println!("{}", random_string);
}

fn generate_random_string(length: u16) -> String {
    let chars: String =
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let chars_vec: Vec<char> = chars.chars().collect();
    let mut random_string = String::new();

    for _ in 0..length {
        let random_number = rand::thread_rng().gen_range(0..chars.len());
        let char = chars_vec.get(random_number).unwrap().to_owned();

        random_string.push(char)
    }

    random_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        let random_string = generate_random_string(10);
        dbg!(&random_string.len());
        assert_eq!(random_string.len(), 10);
    }
}
