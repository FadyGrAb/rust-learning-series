fn main() {
    // Infinity loop
    // loop {}

    loop {
        println!("Enter loop");
        break;
        println!("After break"); // Unreachable
    }
    println!("Outside the loop");

    // Will never break!
    // loop {
    //     continue;
    //     break;  // Unreachable
    // }

    // Returning values from loop
    let mut students_count = 0;

    let class_capacity = loop {
        students_count += 1;

        if students_count >= 30 {
            break students_count;
        }
    };
    println!("The class capacity is {class_capacity}");

    let mut base_count_down = 3;

    'count_down: loop {
        let mut base_count_up = 0;

        'count_up: loop {
            base_count_up += 1;

            if base_count_up > 10 {
                break; // breaks from "count_up loop"
            }

            if base_count_down == 0 {
                break 'count_down; // break from "count_down loop"
            }
            println!("Up: {}, Down: {}", base_count_up, base_count_down);
        }
        base_count_down -= 1;
    }
    println!("Loops are terminated!");

    // while loops
    let mut number = 10;
    while number != 0 {
        println!("The number is {number}");
        number -= 1;
    }
    println!("Outside of while!");

    // For loops
    let arr = [1, 2, 3, 10, 9, 8];

    for element in arr {
        println!("The array value is {element}")
    }

    // count-down loop
    for num in (1..11).rev() {
        println!("Count-down {num}")
    }
    println!("Go! Go! Go!")
}
