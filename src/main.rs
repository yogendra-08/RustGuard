mod integrity;
mod scanner;

use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    println!("================================");
    println!("       RUSTGUARD v1.0");
    println!("    File Integrity Checker");
    println!("================================");

    loop {
        println!("\n1. Create baseline");
        println!("2. Verify directory");
        println!("3. Exit");
        print!("Choose option: ");

        if io::stdout().flush().is_err() {
            eprintln!("Failed to display menu.");
            return;
        }

        let mut choice = String::new();

        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        match choice.trim() {
            "1" => {
                let path = get_directory();

                if let Some(path) = path {
                    if let Err(error) = integrity::create_baseline(&path) {
                        eprintln!("Error creating baseline: {}", error);
                    }
                }
            }
            "2" => {
                let path = get_directory();

                if let Some(path) = path {
                    if let Err(error) = integrity::verify_directory(&path) {
                        eprintln!("Error verifying directory: {}", error);
                    }
                }
            }
            "3" => {
                println!("Exiting RustGuard. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please choose 1, 2, or 3."),
        }
    }
}

fn get_directory() -> Option<PathBuf> {
    print!("Enter folder path to scan: ");

    if io::stdout().flush().is_err() {
        eprintln!("Failed to display prompt.");
        return None;
    }

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read folder path.");
        return None;
    }

    let path = PathBuf::from(input.trim());

    if !path.is_dir() {
        eprintln!("Invalid directory. Please enter a valid folder path.");
        return None;
    }

    Some(path)
}
