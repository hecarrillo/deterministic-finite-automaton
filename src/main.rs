// Import scientific_notation.rs and use its functions
mod scientific_notation;
mod date_notation;
use std::io;
// Create a selectable menu youchoose crate
use youchoose::Menu;

fn main() {
    // Create a menu options 
    let options = vec!["Scientific notation", "Date notation", "Exit"];
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
            println!("Enter a string to check if it is a valid date notation");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let result = date_notation::is_valid_date_notation(input);
            println!("{} is a valid date notation: {}", input, result);
        }
        2 => {
            println!("Goodbye!");
        }
        _ => println!("Invalid choice"),
    }
}






