struct User {
    active: bool,       // stored on the stack
    username: String,   // stored some on stack and the string on heap
    email: String,      // stored some on stack and the string on heap
    sign_in_count: u64, // stored on the stack
}

pub fn demonstrate_structs() {
    println!("=== Struct Examples ===");

    let new_user: User = User {
        active: true,
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 3,
    };
    println!(
        "new user {} of email {} with {} visits and currently {} is created with struct",
        new_user.username, new_user.email, new_user.sign_in_count, new_user.active
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * self.width + 2 * self.height
    }
}

pub fn demonstrate_methods() {
    println!("=== Method Examples ===");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rect1: {}", rect1.area());
    println!("Perimeter of rect1: {}", rect1.perimeter());
}

use std::fmt;

impl fmt::Debug for Rectangle {
    // Rectangle as a struct doesnot by default implement the display trait
    // Below is an example on how a debug trait is implemented on the struct to print out the rect as a whole
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle {{ width: {}, height: {} }}",
            self.width, self.height
        )
    }
}

pub fn demonstrate_debug() {
    println!("=== Debug Examples ===");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1: {:?}", rect1);
}

// unit stucts

struct UnitStruct;

impl UnitStruct {
    fn new(&self) -> u32 {
        0
    }
}

pub fn demonstrate_unit_structs() {
    println!("=== Unit Struct Examples ===");
    let unit_struct = UnitStruct;
    let result = unit_struct.new();
    println!("result: {}", result);
}
