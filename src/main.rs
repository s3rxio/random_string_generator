use rand::Rng;
use std::{
    env,
    fs::{self, File},
    io::{self, prelude::*},
};

const CONFIG_FILE: &str = ".randstrgen";

/* TODO:
- --config-path to specify config file */
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    args.iter().for_each(|arg| match arg.as_str() {
        "-r" | "--reset" => reset_config(),
        _ => print_help(),
    });

    let length: u32;

    if !path_exists(CONFIG_FILE) {
        length = stdin("Enter string length")
            .trim()
            .parse()
            .expect("Enter a valid number!");

        match stdin("Save your choice? (y/n)")
            .trim()
            .to_lowercase()
            .as_str()
        {
            "y" | "yes" => save_config(length).expect("Failed to save config"),
            _ => {}
        };
    } else {
        length = fs::read_to_string(CONFIG_FILE)
            .expect("Failed to read config file")
            .trim()
            .parse()
            .expect("Failed to parse config file");
    }

    println!(
        "Generated string({}): {}",
        length,
        generate_random_string(length)
    );
}

fn print_help() {
    println!("Usage: random_string_generator [length]\nArguments:\n  -h, --help    Show this help message\n  -r, --reset   Reset config file")
}

fn save_config(length: u32) -> io::Result<()> {
    let mut file: File = match path_exists(CONFIG_FILE) {
        true => File::open(CONFIG_FILE)?,
        false => File::create(CONFIG_FILE)?,
    };

    let content = format!("{}", length);

    file.write_all(content.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}

fn reset_config() {
    if !path_exists(CONFIG_FILE) {
        return;
    }
    fs::remove_file(CONFIG_FILE).expect("Failed to remove config file");
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn stdin(to_print: &str) -> String {
    let mut buffer = String::new();

    println!("{}", to_print);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    buffer
}

fn generate_random_string(length: u32) -> String {
    let chars: String =
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let chars_vec: Vec<char> = chars.chars().collect();
    let mut random_string = String::new();

    for _ in 0..length {
        let random_number = rand::thread_rng().gen_range(0..chars.len());
        let char = *chars_vec.get(random_number).unwrap();

        random_string.push(char)
    }

    random_string
}
