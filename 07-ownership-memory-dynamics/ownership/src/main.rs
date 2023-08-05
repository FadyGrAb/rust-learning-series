fn main() {
    let mut hello = String::from("Hello"); // A new String variable using the "from" method from the String namespace.
    hello.push_str(", World!"); // Appends a string to the same String variable.
    println!("{hello}");

    let s1 = String::from("String Value");
    let s2 = s1;
    println!("S2: {s2}");
    // println!("S1: {s1}"); // Error borrow of moved value: `s1`

    let n1 = 10;
    let n2 = n1;
    println!("n1: {n1}"); // is valid.
    println!("n2: {n2}");

    let s3 = String::from("Clone");
    let s4 = s3.clone();
    println!("s3: {s3}"); // is valid.
    println!("s4: {s4}");
}
