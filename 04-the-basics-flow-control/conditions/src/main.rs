fn main() {
    let age: u8 = 29;

    // if age { // Error: expected `bool`, found integer.
    if age < 30 {
        println!("Age is below 30");
    } else if age > 30 {
        println!("Age is above 30");
    } else {
        println!("Age is equal to 30");
    }

    // Using if with let
    let threshold = 50;
    let value = 10;
    let effective_value = if value > threshold {
        value - threshold
    } else {
        0
    };
    // let effective_value = if value > threshold {   // Error: expected integer, found '&str'
    //     value - threshold
    // } else {
    //     "zero"
    // };
    println!("Effective value is {effective_value}")
}
