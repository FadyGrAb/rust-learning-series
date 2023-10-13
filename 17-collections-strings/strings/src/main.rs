fn main() {
    // Create new String
    let my_string: String = String::new();
    let my_string: String = String::from("Hello");
    println!("from: {my_string}");
    let word: &str = "Hello";
    let my_string: String = word.to_string();
    println!("to_string: {my_string}");

    // Updating a String
    let mut new_str = String::from("Hello, ");
    new_str.push_str("There");
    println!("Greeting: {new_str}");
    new_str.push('!');
    println!("New greeting: {new_str}");
    // updating with +
    let s1 = String::from("Hello,");
    let s2 = String::from("There!");
    let s3 = s1 + &s2;
    println!("s3 is {s3}");

    let s1 = String::from("Hello");
    let s2 = String::from("There");
    let s3 = String::from("!");
    let s4 = s1 + ", " + &s2 + &s3;
    println!("s4 is {s4}");

    // Using format! macro
    let s1 = String::from("Hello");
    let s2 = String::from("There");
    let s3 = String::from("!");
    let s4 = format!("{s1}, {s2}{s3}");
    println!("s4 is {s4}");

    // Indexing into Strings
    let my_string = String::from("Greetings 😉 !");
    // let G = my_string[0]; // the type `String` cannot be indexed by `{integer}`
    let Gree = &my_string[..4];
    println!("{Gree}");
    // let wink = &my_string[9..12];  // app will panic!
    // println!("{wink}");

    // Iterating over strings:
    for c in my_string.chars() {
        println!("{c}")
    }
    for b in my_string.as_bytes() {
        println!("{b}")
    }
}
