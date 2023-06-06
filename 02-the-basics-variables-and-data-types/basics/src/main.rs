fn main() {
    // VARIABLES:
    // let x = 5;   // Variables are immutable by default
    // x = 6;       // Error: cannot assign twice to immutable variable `x`
    let mut x = 5;
    println!("The value of x before assignment is {x}");
    x = 6; // This won't error out as x is mutable now.
    println!("The value of x is {x}");

    // CONSTANTS:
    // const WORK_HOURS_PER_WEEK = user_input;  // Invalid constant
    const MINUTES_IN_DAY: u32 = 1 * 24 * 60; // Valid constant
    println!("The value for MINUTES_IN_DAY is {MINUTES_IN_DAY}");

    // Data Types
    // let guess = "42".parse().expect("Not a number!"); // Error: type annotations needed
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value for \"guess\" is {guess}");
    // Charcter type
    let cat = '🐈';
    println!("This is a cat {cat}");
    // Tuples
    let ip_address: (u8, u8, u8, u8) = (168, 1, 1, 10);
    let (a, b, c, d) = ip_address;
    println!("The IP address is {a}.{b}.{c}.{d}");
    let user: (&str, u8, char, bool) = ("Fady", 37, 'm', true);
    println!("name: {}, gender: {}", user.0, user.2);
    // Arrays
    let number = [1, 2, 3, 4];
    println!("The second number is {}", number[1]);
    let a = [10; 3]; // An array of size 3 of 10 value
    let hours = [1; 24];
    // println!("{}", hours[25]) // Error: index out of bounds: the length is 24 but the index is 25
}
