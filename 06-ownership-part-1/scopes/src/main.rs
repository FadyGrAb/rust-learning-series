fn main() {
    // Start of "main" function scope.
    let x = 5;
    println!("Value of x from 'main' scope is {x}");

    {
        // Start of "internal" scope.
        let y = 10;
        println!("Value of y from internal scope is {y}");
        println!("Value of x from 'internal' scope again is {x}");
    } // End of "internal" scope.

    // println!("Value of y from 'main' scope is {y}"); // Error: cannot find value `y` in this scope
    println!("Value of x from 'main' scope again is {x}");
} // End of "main" function scope
