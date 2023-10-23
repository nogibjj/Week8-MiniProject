use std::env; // Import the env module.
use caesar_cipher::decrypt;
use caesar_cipher::encrypt;

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect the command-line arguments.

    if args.len() != 3 {
        eprintln!("Usage: cargo run <plaintext> <shift>");
        std::process::exit(1); // Exit with an error code if the number of arguments is incorrect.
    }

    let plaintext = &args[1]; // Get the plaintext from the command-line arguments.

    // Attempt to parse the shift as a u8. Exit with an error code if it fails.
    let shift: u8 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Shift value must be a positive integer");
            std::process::exit(1);
        }
    };

    // Ensure the shift is a positive integer less than 26.
    if shift >= 26 {
        eprintln!("Error: Shift value must be a positive integer less than 26");
        std::process::exit(1);
    }

    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Decrypted text: {}", decrypted_text);
}
