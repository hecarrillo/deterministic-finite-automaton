// Import scientific_notation.rs and use its functions
mod scientific_notation;
mod pair_zeros;
mod sucessive_letters;
mod date_notation;
use std::io;
// Create a selectable menu youchoose crate
use youchoose::Menu;

fn print_accepted() {
    println!("⬇️");
    println!("ACCEPTED");
}

fn print_rejected() {
    println!("⬇️");
    println!("REJECTED");
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn print_menu() -> usize {
    // Create a menu options 
    let options = vec!["Scientific notation","Pair of zeros","Sucessive letters", "Date notation", "Exit"];
    // Create a menu passing an Iterator of options
    let mut menu = Menu::new(options.iter());
    let choice: Vec<usize> = menu.show();
    // parse choice to integer
    let choice_as_int = choice[0];
    print!("{}[2J", 27 as char);
    choice_as_int
} 

fn main() {
    loop {
        let choice = print_menu();
        if choice == 4 {
            break;
        }
        println!("Enter a string to validate:");
        let input: String = read_user_input();
        match choice {
            0 => {
                if scientific_notation::is_valid(&input) {
                    print_accepted();
                } else {
                    print_rejected();
                }
            },
            1 => {
                if pair_zeros::is_valid(&input) {
                    print_accepted();
                } else {
                    print_rejected();
                }
            },
            2 => {
                if sucessive_letters::is_valid(&input) {
                    print_accepted();
                } else {
                    print_rejected();
                }
            },
            3 => {
                if date_notation::is_valid(&input) {
                    print_accepted();
                } else {
                    print_rejected();
                }
            },
            _ => {
                println!("Invalid choice");
            }
        }
        read_user_input();
    }
}






