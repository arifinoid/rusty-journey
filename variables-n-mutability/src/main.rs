fn main() {
    let x = 5;
    println!("The value of x is {x} and immutable by default");

    let mut y = 10;
    println!("The value of y is {y} and mutable");
    y += 1;
    println!("Now the value of y is {y}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {THREE_HOURS_IN_SECONDS} seconds");

    {
        // shadowing
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length: {spaces}")
}
