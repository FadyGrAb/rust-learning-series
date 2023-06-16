use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to: GUESS THE NUMBER game!");

    // Generate the secret number and initiate the tries.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;

    // Main game infinite loop.
    loop {
        println!("Please input your guess ...");

        let mut guess = String::new(); // guess is a String type.
        tries += 1; // Increment the tries at each loop iteration.

        // Get input from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to parse input!");

        let guess: &str = guess.trim(); // Shadowing. guess is now a string literal.

        // Check the for quit
        if guess.to_lowercase() == "quit" {
            break;
        }

        // Parse user input.
        let guess: u32 = match guess.trim().parse() {
            // More shadowing, guess is a u32 now upon successful parsing.
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number between 1 and 100");
                continue;
            }
        };

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Took you {tries} tries to guess the secret number!");
                break;
            }
        }
    }
}
