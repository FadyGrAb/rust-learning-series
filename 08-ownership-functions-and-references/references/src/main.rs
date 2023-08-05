fn main() {
    let s1 = String::from("references");
    let len = return_length(&s1);
    println!("s1: '{s1}' and its length is {len}"); // won't error out as s1 wasn't moved into 'return_length'

    // Mutable reference
    let mut word = String::from("Hello");
    add_hello(&mut word);
    println!("word: {word}");

    let mut s = String::from("Mutable");
    let s1 = &mut s;

    // ERROR: cannot borrow `s` as mutable more than once at a time
    // let s2 = &mut s;
    // println!("s1: {s1}, s2 {s2}");

    let mut r = String::from("ref");
    let r1 = &r; // no problem
    let r2 = &r; // no problem

    // ERROR: cannot borrow `r` as mutable because it is also borrowed as immutable
    // let r3 = &mut r; // problem
    // println!("{r1} {r2} {r3}");

    let mut x = String::from("new ref");
    let x1 = &x; // no problem
    let x2 = &x; // no problem
    println!("{x1}, {x2}");
    let x3 = &mut x; // It's OK now.
    println!("{x3}");
}

fn return_length(s: &String) -> usize {
    s.len()
}

fn add_hello(s: &mut String) {
    s.push_str(", World!");
}
