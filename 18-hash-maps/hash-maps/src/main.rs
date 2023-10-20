use std::collections::HashMap;
fn main() {
    // Create a Hash Map
    let mut map: HashMap<String, u8> = HashMap::new();

    map.insert(String::from("Bob"), 30);
    map.insert(String::from("Ben"), 25);
    println!("{map:?}");

    // Accessing values
    let bob_score = map.get("Bob").copied().unwrap_or(0);
    println!("Bob scored {bob_score}");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Hash Maps and Ownership
    let jane = String::from("Jane");
    let jane_score = 29;
    map.insert(jane, jane_score);

    // println!("{jane} scored {jane_score}"); // Error: borrow of moved value: `jane`

    // Updating a Hash Map
    // 1- Overwriting a value:
    let jane_score = map.get("Jane").copied().unwrap();
    println!("Jane score before the update {jane_score}");
    map.insert(String::from("Jane"), 30);
    let jane_score = map.get("Jane").copied().unwrap();
    println!("Jane score after the update {jane_score}");

    // 2- Adding a key and value if the key isn't present:
    map.entry(String::from("Jane")).or_insert(25);
    map.entry(String::from("Windy")).or_insert(25);
    println!("{map:?}");

    // 3- Updated values based on old values
    for (_, score) in map.iter_mut() {
        *score += 5;
    }
    println!("{map:?}");
}
