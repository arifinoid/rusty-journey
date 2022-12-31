fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    multiple_mutable_references();

    // let reference_to_nothing = dangle();
    no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
  //So, what happens if we try to modify something we’re borrowing? it doesn’t work!

// Mutable references
// We can fix the code from above to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // Mutable references have one big restriction: if you have a mutable reference to a value,
    // you can have no other references to that value. This code that attempts to create two mutable references to s will fail
}

// The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.
// Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{r2}");

    // Rust enforces a similar rule for combining mutable and immutable references.This code results in an error:
    // let mut s = String::from("hello");

    // let r3 = &s; // no problem
    // let r4 = &s; // no problem
    // let r5 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r3, r4, r5);
    //
    // but we can fix them like so

    let r3 = &s; // no problem
    let r4 = &s; // no problem
    println!("{} and {}", r3, r4);
    // variables r1 and r2 will not be used after this point

    let r5 = &mut s; // no problem
    println!("{}", r5);
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!u

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it.
// That means this reference would be pointing to an invalid String.
// That’s no good! Rust won’t let us do this.

// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
// This works without any problems. Ownership is moved out, and nothing is deallocated.

// The Rules of References
// Let’s recap what we’ve discussed about references:
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.
