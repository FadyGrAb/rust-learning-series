fn main() {
    // Using the Option Enum without importing it.
    let some_number = Some(5);
    let number = 5;
    let name = Some("My Name");
    let none: Option<i32> = None;

    // Some Option Enum methods
    assert_eq!(some_number.is_some(), true);
    // assert_eq!(none.is_none(), false);

    // Some(i32) and i32 are not the same. This will produce compile error.
    // let addition = some_number + number;

    // Using the data bound to Some.
    let five = Some(5);

    let mut result = double(five);
    println!("Double of 5 is: {result:?}");

    result = double(none);
    println!("Double of None is: {result:?}");

    // Using IF Let.
    let reading: Option<u8> = Some(5);

    match reading {
        Some(5) => println!("Reading of 5 is received [match]"),
        _ => (),
    }

    if let Some(5) = reading {
        println!("Reading of 5 is received [if let]")
    }
}

fn double(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n * 2),
        None => None,
    }
}
