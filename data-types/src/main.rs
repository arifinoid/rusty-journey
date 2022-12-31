use std::io;

fn main() {
    // scalar types
    // integers, floating-point numbers, booleans, and characters
    let x = 2.0; // f64
    let y: f32 = 3.0; // f64

    println!("x: {x}, y: {y}");

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum}, diff: {difference}, product: {product}, quot: {quotient}, truncated: {truncated}, remainder: {remainder}");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t: {t}, f: {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // Compound types
    // tuples and arrays
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of x is {x}, y is {y}, and z is {z}");

    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "given array a: {:?}, please choose a number inside this array by type the index",
        a
    );

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");
    let element = a[index];
    println!("the value of the element at index {index} is: {element}")
}
