// Import scientific_notation.rs and use its functions
mod scientific_notation;
mod pair_zeros;
mod sucessive_letters;
mod date_notation;
use std::io;
// Create a selectable menu youchoose crate
use youchoose::Menu;

fn main() {
    // Create a menu options 
    let options = vec!["Scientific notation","Pair of zeros","Sucessive letters", "Date notation", "Exit"];
    // Create a menu passing an Iterator of options
    let mut menu = Menu::new(options.iter());
    let choice: Vec<usize> = menu.show();
    // parse choice to integer
    let choice_as_int = choice[0];
    print!("{}[2J", 27 as char);
    // match the choice with the functions
    match choice_as_int {
        0 => {
            println!("Enter a string to check if it is a valid scientific notation");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let result = scientific_notation::is_valid_scientific_notation(input);
            println!("{} is a valid scientific notation: {}", input, result);
        }
        1 => {
        println!("Enter a string to check if it is a string with pair number of zeros and without ones in a row");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let result = pair_zeros::is_valid_pair_zeros(input);
            println!("{} is a valid string: {}", input, result);
        }
        2 => {
            println!("Enter a string to check if it is a string with at least two succesive same letters");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let result = sucessive_letters::is_valid_sucessive_letters(input);
            println!("{} is a valid string: {}", input, result);
        }
        3 => {
            println!("Enter a string to check if it is a valid date notation");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let result = date_notation::is_valid_date_notation(input);
            println!("{} is a valid date notation: {}", input, result);
        }
        4 => {
            println!("Goodbye!");
        }
        _ => println!("Invalid choice"),
    }
}






