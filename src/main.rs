// Import scientific_notation.rs and use its functions
mod scientific_notation;
use std::io;

// read user input 
fn get_user_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn main() {
    // prompt a menu to the user and get the user input
    println!("Enter a string to check if it is a valid scientific number notation (e.g. 1.23e-4, 1.23e+4, 1.23e4, 1.23E-4, 1.23E+4, 1.23E4)");
    let input = get_user_string();
    // Check if input is a valid scientific number notation
    let result = scientific_notation::is_valid_scientific_notation(&input);
    if result {
        println!("Valid scientific number notation");
    } else {
        println!("Invalid scientific number notation");
    }
}

