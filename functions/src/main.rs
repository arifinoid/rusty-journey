fn main() {
    let increment = plus_one(1);
    println!("the result is {increment}")
}

fn plus_one(num: i32) -> i32 {
    num + 1 // or return num + 1;
}
