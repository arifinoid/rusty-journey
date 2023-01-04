#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32); // Using Tuple Structs without Named Fields

// We use structs to add meaning by labeling the data.
// We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining Methods
// defined within the context of a struct, and their first parameter is always self,
// which represents the instance of the struct the method is being called on
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is not a method. this is associated functions, because  they’re associated with the type named after the impl.
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
    // We’ve already used one function like this: the String::from function that’s defined on the String type.
    //
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    // These are often called new, but new isn’t a special name and isn’t built into the language.
    // For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height,
    // thus making it easier to create a square Rectangle rather than having to specify the same value twice
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn create_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user_one = User {
        email: String::from("admin@test.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };

    let user_two = create_user(String::from("user@test.com"), String::from("user"));
    let user_three = User {
        email: String::from("user.three@test.com"),
        username: String::from("user_three"),
        ..user_one
    };

    let black = Color(0, 0, 0);

    println!("{:#?}", user_one);
    println!("{:#?}", user_two);
    println!("{:#?}", user_three);
    println!("{:#?}", black);

    let rect = (30, 50);
    println!("the area of rectangle is {} square pixels.", area(rect));

    let another_rect = Rectangle {
        height: 30,
        width: 50,
    };

    let yet_another_rect = Rectangle {
        height: 20,
        width: 40,
    };

    println!(
        "the area of another rectangle is {} square pixels.",
        another_area(&another_rect)
    );

    // calling method
    println!(
        "The area of the rectangle is {} square pixels.",
        another_rect.area()
    );

    println!(
        "Can rect1 hold rect2? {}",
        another_rect.can_hold(&yet_another_rect)
    );

    // calling associated function
    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn another_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
