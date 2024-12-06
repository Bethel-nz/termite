mod utils;
use std::{thread, time::Duration};

use crate::utils::{get_hasher, HashingUtil};
use inquire::{validator::Validation, Select, Text};
use rand::Rng;
use spinners::{Spinner, Spinners};

fn main() {
    println!(
        "
      _____ _____ ____  __  __ ___ _____ _____
     |_   _| ____|  _ \\|  \\/  |_ _|_   _| ____|
       | | |  _| | |_) | |\\/| || |  | | |  _|
       | | | |___|  _ <| |  | || |  | | | |___
       |_| |_____|_| \\_\\_|  |_|___| |_| |_____|
    "
    );
    println!("A simple CLI tool for secure string hashing");

    let string_validator = |input: &str| {
        if input.chars().count() <= 6 {
            Ok(Validation::Invalid(
                "Termite needs input to be at least 6 characters".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    const DEFAULT_ALGO: &str = "SHA256";

    let algorithms = vec![
        (DEFAULT_ALGO, "Modern", "High Security"),
        ("BLAKE3", "Modern", "Very Fast"),
        ("MD5", "Legacy", "Not Recommended"),
    ];

    let available_algos: Vec<String> = algorithms
        .iter()
        .map(|(name, _, _)| format!("{} ", name))
        .collect();

    let input = Text::new("Enter your string to hash:")
        .with_validator(string_validator)
        .prompt()
        .unwrap();

    let selected_algorithms = Select::new("Choose algorithm:", available_algos)
        .with_help_message("↑↓ to move, enter to select, SHA256 is default")
        .prompt()
        .unwrap_or("SHA256 (Modern - High Security)".to_string());

    let key = generate_random_key();
    let hasher: Box<dyn HashingUtil> = get_hasher(&selected_algorithms);
    let hashed = hasher.hash_string(&input, &key);
    let mut spinner = Spinner::new(
        Spinners::Dots,
        format!("Generating hash using {}...", selected_algorithms.trim()).into(),
    );

    thread::sleep(Duration::from_secs(2));
    let output = format!(
        r#"
Hash generated with {}!

Results:
--------
Key: {}
Hash: {}

"#,
        selected_algorithms.trim(),
        key,
        hashed
    );

    spinner.stop_with_message(output.into());
}

// Generates Random Keys to be used for hashing
fn generate_random_key() -> String {
    let char_map = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9", "!", "@", "#", "$", "%", "^", "&", "*",
    ];

    let mut rng = rand::thread_rng();
    let key: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..char_map.len());
            char_map[idx]
        })
        .collect();

    key
}
