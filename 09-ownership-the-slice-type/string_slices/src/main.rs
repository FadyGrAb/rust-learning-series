fn main() {
    let s = String::from("Hello this is a test sentence");
    let word1 = &s[0..5];
    let word2 = &s[..5];
    println!(
        "The first word is word1: '{}' and is also word2: '{}'",
        word1, word2
    );
    let ln = s.len();
    let word3 = &s[21..ln];
    let word4 = &s[21..];
    println!(
        "The last word is word3: '{}' and is also word4: '{}'",
        word3, word4
    );
    let word5 = &s[..];
    println!(
        "The whole sentence is s: '{}' and is also word5: '{}'",
        s, word5
    )
}
