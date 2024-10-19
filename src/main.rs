mod error;

use clap::Parser;
use error::{Error, Result};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version="0.1.0", about="random_string_generator", long_about = None)]
struct Options {
    #[arg(short = 'l', long = "length", help = "Length of the generated string")]
    length: u32,

    #[arg(short = 'o', long = "output", help = "Path to the output file")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Options::parse();
    let random_string = rand::thread_rng().gen_string(args.length);

    match args.output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(random_string.as_bytes())?;
        }
        None => println!("{}", random_string),
    }

    Ok(())
}

trait GenString {
    fn gen_string(&mut self, max: u32) -> String;
}

impl GenString for rand::rngs::ThreadRng {
    fn gen_string(&mut self, max: u32) -> String {
        let chars: Vec<char> =
            String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
                .chars()
                .collect();
        let mut generated = String::new();

        let mut rng = rand::thread_rng();
        for _ in 0..max {
            let char = chars.choose(&mut rng).unwrap();
            generated.push(*char);
        }

        generated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_ten() {
        let string = rand::thread_rng().gen_string(10);

        assert_eq!(string.len(), 10);
    }

    #[test]
    fn should_generate_bigger() {
        let string = rand::thread_rng().gen_string(128);

        assert_eq!(string.len(), 128);
    }

    #[test]
    fn should_generate_max() {
        let string = rand::thread_rng().gen_string(u32::MAX);

        assert_eq!(string.len() as u32, u32::MAX);
    }
}
