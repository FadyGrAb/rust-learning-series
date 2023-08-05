fn main() {
    let s1 = String::from("string");
    take_scope(s1); // s1 is moved into 'take_scope'

    // Error: borrow of moved value: `s1`
    //println!("s1: {s1}")

    let n1 = 5;
    take_num(n1); // n1 is COPIED into 'take_num'
    println!("n1 from main: {n1}"); // this is fine as n1 is copied not moved.

    let s1 = String::from("String 2"); // Valid as s1 is already moved to 'take_scope'
    let s2 = take_and_return_scope(s1); // s1 is moved to 'take_and_return_scope' then moved back into s2
    println!("S2: {s2}");

    let s3 = String::from("String 3");
    let (s4, len) = return_length(s3); // 'return_length' moves back the original string value along with the computed value.
    println!("s4 is '{s4}', length is {len}");
}

fn take_scope(s: String) -> () {
    println!("s1 from take_scope {s}")
}

fn take_num(n: i32) -> () {
    println!("n: {n}")
}

fn take_and_return_scope(s: String) -> String {
    s
}

fn return_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s, length)
}
