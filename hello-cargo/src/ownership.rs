// ownership
// ownership is a set of rules how a rust program manages memory.
// all programs have to manage the way they use a computers memory while running
// some langs have GC that regularly looks for no-longer-needed memory as the program runs,
// in other langs the programmer must explicitly allocate and free the memory.
// Rust uses a third approach, memory is managed through a system of ownership whit a set of rules that the compiler checks
// If any of the rules are violated, the program will not compile.
// none of the features of ownership will slow down the program while its running.

//Stack variables
// examle 1 - passing stack variables inside functions

pub fn vars_in_functions() {
    let x = 5;
    let y = 10;
    let result = add(x, y);
    println!("The sum of {} and {} is {}", x, y, result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// example 2 -  scoping variables in the same function
pub fn scope_in_function() {
    let x = 5;
    {
        let y = 10;
        println!("Inside inner scope: x = {}, y = {}", x, y);
    }
    // println!("Outside inner scope: y = {}", y); // This would cause a compile error!
    println!("Outside inner scope: x = {}", x);
}

// heap variables

// example 1 passing strings (heap variables) to functions as args

pub fn strings_in_functions() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let result = concatenate(s1, s2);
    println!("Concatenated string: {}", result);
}

fn concatenate(s1: String, s2: String) -> String {
    format!("{} {}", s1, s2)
}

pub fn strings_in_funtions_2() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(my_string);
    // println!("my_string: {}", my_string); // This would cause a compile error!
    // println!("{}", my_string);
}

fn takes_ownership(some_string: String) {
    println!("Received string: {}", some_string);
}

// at any time, each value can have a single owner
// this is to avoid memory issues like double free error or dangling pointers
// when the owner goes out of scope, the value will be dropped
