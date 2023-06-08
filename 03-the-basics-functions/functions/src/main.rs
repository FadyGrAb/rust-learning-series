fn main() {
    println!("Main Function");
    // A function call
    second_function();
    // A function with parameters
    print_temperature(35, 'C');

    // Statements and Expressions
    // let y = (let z = 10); // Error: expected expression, found statement
    let x = 10; // Statement
    println!("The values of x is {x}");

    let y = {
        let z = 10;
        z + 13
    }; // The whole block is an expression
    println!("The value of y is {y}");

    //Return Values
    let number_five = five();
    println!("number_five's value is {number_five}");

    let add_one = add_one(7);
    println!("The value of add_one is {add_one}")
}

fn second_function() {
    println!("Second Function");
}

fn print_temperature(value: i32, unit: char) {
    println!("The temperature is {value}{unit}");
}

fn five() -> u8 {
    5
}

fn add_one(value: i32) -> i32 {
    // value + 1; // Error (notice the semi-colon). The function is expecting a i32 return type but got unit ()
    value + 1
}
