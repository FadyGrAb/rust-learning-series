fn main() {
    // Creating a new vector
    let my_vector: Vec<i32> = Vec::new(); // Create and empty *immutable* vector.
    let my_other_vector = vec![1, 2, 3]; // Omitting the type as Rust will infer it.

    // Updating a vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Length of v is: {}", v.len());
    v.pop();
    println!("New length of v is: {}", v.len());

    // Reading values in vectors
    let first_element = &v[0]; // Square brackets notations
    println!("First element is {first_element}");
    let second_element: Option<&i32> = v.get(1); // Using get
    match second_element {
        Some(second) => println!("Second element is {second}"),
        None => println!("There isn't a second element!"),
    }

    let v2 = vec![1, 2, 3];
    //let val = &v2[100]; // Program panics.
    let val = v2.get(100); // Returns None
    if let None = val {
        println!("None is returned!")
    }

    // Vectors and Ownership
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    // println!("{first}"); // Borrowing error.

    // Iterating over vectors
    for i in &v {
        println!("{i}");
    }

    // Changing vector elements while iterating over it
    let mut v2 = vec![20, 40, 60];
    for i in &mut v2 {
        *i -= 20;
    }
    println!("Modified vector:");
    for i in &v2 {
        println!("{i}")
    }

    // Using Enum to make the vectors contain multiple data types
    #[derive(Debug)]
    enum Item {
        Apple(u8),
        Banana(u8),
        Tape(f32),
        Book(String),
    }

    let shopping_cart = vec![
        Item::Apple(10),
        Item::Banana(5),
        Item::Book(String::from("Moby-Dick")),
        Item::Tape(5.5),
    ];
    println!("Items in the shopping cart:");
    for item in &shopping_cart {
        println!("{item:?}");
    }
}
