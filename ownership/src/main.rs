fn main() {
    run_from_stack();
    run_from_heap();
    // So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory

    move_value();
    clone_value();

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.
    // These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still use x afterward

    // Returning values can also transfer ownership.
    scope_n_ownership();

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of {s2} is {len}")
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn run_from_stack() {
    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward

    println!("the value of s is: {s}"); // do stuff with s
} // this scope is now over, and s is no longer valid

fn run_from_heap() {
    let mut s = String::from("hello"); // manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn move_value() {
    // This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory
    // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
    // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    let s1 = String::from("hello");
    let s2 = s1;
    //  Rust invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2
    //  With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done

    println!("{}, world!", s2);
}

fn clone_value() {
    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // This works just fine and explicitly produces the behavior produces different pointer but same data,
    // where the heap data does get copied.

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn scope_n_ownership() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("s1: {s1}, s3: {s3}")
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// this function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
