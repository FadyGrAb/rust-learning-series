fn main() {
    // Will result in compile error.
    let mut s = String::from("Hello this is a test sentence");
    let word = find_word(&s);
    println!("Sentence is '{s}' before clear");
    s.clear();
    println!("First word is {word}");
    println!("Sentence is '{s}' after clear.");
}
fn find_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();

    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &sentence[..idx];
        }
    }
    &sentence[..]
}
