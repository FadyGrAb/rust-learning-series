fn main() {
    let mut s = String::from("Hello this is a test sentence");
    let index = find_word(&s);
    println!("Sentence is '{s}' before clear");
    s.clear();
    println!("Sentence is '{s}' after clear.");
    println!("The first word ends at index: {}", index - 1);
}

fn find_word(sentence: &String) -> usize {
    let bytes = sentence.as_bytes();
    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return idx;
        }
    }
    sentence.len()
}
